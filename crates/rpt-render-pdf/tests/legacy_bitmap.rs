//! Synthetic image regressions for the opt-in legacy BMP PDF color path.
use rpt_model::{Rect, Twips};
use rpt_pages::{DrawOp, ImageAsset, ImageFit, ImageOp, Page, PageSize, PagedDocument};
use rpt_render_pdf::{try_render_document, PdfOptions};
use rpt_test_support::pdf::{image_rgb8, operator_listing, structure};

fn pixels() -> Vec<[u8; 3]> {
    let bases = [[16u8, 80, 40], [200, 40, 80], [40, 56, 216]];
    (0..7)
        .flat_map(|y| {
            (0..7).map(move |x| {
                let base = bases[(x + y) % 3];
                let shade = (3 * x + y) % 4;
                std::array::from_fn(|i| base[i] + ((shade + i) % 4) as u8)
            })
        })
        .collect()
}

fn bitmap(top_down: bool, bpp: usize) -> Vec<u8> {
    let stride = (7 * (bpp / 8) + 3) & !3;
    let mut data = vec![0; 54 + 7 * stride];
    data[..2].copy_from_slice(b"BM");
    data[10..14].copy_from_slice(&54u32.to_le_bytes());
    data[14..18].copy_from_slice(&40u32.to_le_bytes());
    data[18..22].copy_from_slice(&7i32.to_le_bytes());
    data[22..26].copy_from_slice(&(if top_down { -7i32 } else { 7 }).to_le_bytes());
    data[26..28].copy_from_slice(&1u16.to_le_bytes());
    data[28..30].copy_from_slice(&(bpp as u16).to_le_bytes());
    for (i, rgb) in pixels().iter().enumerate() {
        let y = i / 7;
        let row = if top_down { y } else { 6 - y };
        let p = 54 + row * stride + (i % 7) * (bpp / 8);
        data[p..p + 3].copy_from_slice(&[rgb[2], rgb[1], rgb[0]]);
    }
    data
}

fn document(bytes: Vec<u8>, media: &str) -> PagedDocument {
    let mut page = Page::new(
        1,
        PageSize {
            width: Twips(3000),
            height: Twips(3000),
        },
    );
    for x in [200, 1400] {
        page.push(DrawOp::Image(ImageOp {
            bounds: Rect {
                left: Twips(x),
                top: Twips(200),
                width: Twips(1000),
                height: Twips(1000),
            },
            image_id: "synthetic".into(),
            fit: ImageFit::Fill,
            source: None,
        }));
    }
    let mut second = page.clone();
    second.number = 2;
    PagedDocument {
        pages: vec![page, second],
        assets: [(
            "synthetic".into(),
            ImageAsset {
                media_type: media.into(),
                bytes,
            },
        )]
        .into_iter()
        .collect(),
        ..Default::default()
    }
}

fn rgb(pdf: &[u8]) -> Vec<u8> {
    let parsed = structure(pdf);
    assert_eq!(parsed.pages.len(), 2);
    let ids: Vec<_> = parsed
        .pages
        .iter()
        .map(|page| {
            assert_eq!(
                page.images.len(),
                1,
                "repeated bitmap should share one XObject"
            );
            let image = page.images.values().next().unwrap();
            assert_eq!((image.width, image.height), (7, 7));
            image.object
        })
        .collect();
    assert_eq!(ids[0], ids[1], "image should be reused across pages");
    image_rgb8(pdf, ids[0]).expect("plain RGB8 image")
}
fn hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x100000001b3)
    })
}

#[test]
fn default_preserves_pixels_and_opt_in_matches_native_synthetic_output() {
    let doc = document(bitmap(false, 24), "image/bmp");
    let normal = try_render_document(&doc, &PdfOptions::default()).unwrap();
    assert!(!PdfOptions::default().legacy_bmp_quantization);
    assert_eq!(
        rgb(&normal),
        pixels().into_iter().flatten().collect::<Vec<_>>()
    );
    let enabled = try_render_document(
        &doc,
        &PdfOptions {
            legacy_bmp_quantization: true,
            ..Default::default()
        },
    )
    .unwrap();
    // Independent native-generated result for the synthetic 7x7 phased colors, with all lookup
    // bins explicitly written. This constant is not calculated by the production implementation.
    assert_eq!(hash(&rgb(&enabled)), 0x97c3_ff44_2ae9_56ca);
    assert_ne!(rgb(&normal), rgb(&enabled));
    assert_eq!(
        operator_listing(&normal),
        operator_listing(&enabled),
        "image color conversion must not move any drawing"
    );
    assert_eq!(
        normal,
        try_render_document(
            &doc,
            &PdfOptions {
                legacy_bmp_quantization: false,
                ..Default::default()
            }
        )
        .unwrap()
    );
}

#[test]
fn uncharacterized_bmp_storage_keeps_normal_pixels() {
    for (top_down, bpp) in [(true, 24), (false, 32), (true, 32)] {
        let doc = document(bitmap(top_down, bpp), "image/bmp");
        let normal = try_render_document(&doc, &PdfOptions::default()).unwrap();
        let enabled = try_render_document(
            &doc,
            &PdfOptions {
                legacy_bmp_quantization: true,
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(normal, enabled);
        assert_eq!(
            rgb(&enabled),
            pixels().into_iter().flatten().collect::<Vec<_>>()
        );
    }
}
