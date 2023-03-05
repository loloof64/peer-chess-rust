use iced_native::svg;
use iced_native::svg::Handle;

#[derive(Clone)]
pub struct PiecesVectors {
    pub svg_wp: Handle,
    pub svg_wn: Handle,
    pub svg_wb: Handle,
    pub svg_wr: Handle,
    pub svg_wq: Handle,
    pub svg_wk: Handle,
    pub svg_bp: Handle,
    pub svg_bn: Handle,
    pub svg_bb: Handle,
    pub svg_br: Handle,
    pub svg_bq: Handle,
    pub svg_bk: Handle,
}

impl PiecesVectors {
    pub fn new() -> Self {
        let svg_wp = svg::Handle::from_path(format!(
            "{}/resources/images/chess_vectors/Chess_plt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_wn = svg::Handle::from_path(format!(
            "{}/resources/images/chess_vectors/Chess_nlt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_wb = svg::Handle::from_path(format!(
            "{}/resources/images/chess_vectors/Chess_blt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_wr = svg::Handle::from_path(format!(
            "{}/resources/images/chess_vectors/Chess_rlt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_wq = svg::Handle::from_path(format!(
            "{}/resources/images/chess_vectors/Chess_qlt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_wk = svg::Handle::from_path(format!(
            "{}/resources/images/chess_vectors/Chess_klt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_bp = svg::Handle::from_path(format!(
            "{}/resources/images/chess_vectors/Chess_pdt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_bn = svg::Handle::from_path(format!(
            "{}/resources/images/chess_vectors/Chess_ndt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_bb = svg::Handle::from_path(format!(
            "{}/resources/images/chess_vectors/Chess_bdt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_br = svg::Handle::from_path(format!(
            "{}/resources/images/chess_vectors/Chess_rdt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_bq = svg::Handle::from_path(format!(
            "{}/resources/images/chess_vectors/Chess_qdt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg_bk = svg::Handle::from_path(format!(
            "{}/resources/images/chess_vectors/Chess_kdt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));

        Self {
            svg_wp,
            svg_wn,
            svg_wb,
            svg_wr,
            svg_wq,
            svg_wk,
            svg_bp,
            svg_bn,
            svg_bb,
            svg_br,
            svg_bq,
            svg_bk,
        }
    }
}
