use bocks::Bocks;
use Dims;
use TTYStyle;
#[derive(Debug)]
pub struct Rule {
    pub left_box: Bocks,
    pub right_box: Bocks,
    pub left_box_tty_vidd: usize,
    pub right_box_tty_vidd: usize,
    pub left_box_tty_hojd: usize,
    pub right_box_tty_hojd: usize,
}
impl Rule {
    pub fn new(left: Bocks, right: Bocks) -> Rule {
        Rule {
            left_box: left,
            right_box: right,
            left_box_tty_vidd: 0,
            right_box_tty_vidd: 0,
            left_box_tty_hojd: 0,
            right_box_tty_hojd: 0,
        }
    }
    /*................................................*/
    pub fn calc_vidder(&mut self) {
        self.left_box_tty_vidd = self.left_box.calc_vidd();
        self.right_box_tty_vidd = self.right_box.calc_vidd();
    }
    /*................................................*/
    pub fn calc_hojder(&mut self) {
        self.left_box_tty_hojd = self.left_box.calc_hojd();
        self.right_box_tty_hojd = self.right_box.calc_hojd();
    }
    /*................................................*/
    pub fn _dump(&mut self) {
        self.left_box._dump();
        self.right_box._dump();
    }
    /*................................................*/
    pub fn as_tty(
        &mut self,
        start_x: usize,
        start_y: usize,
        max_left_rule: usize,
        dims_tty: &TTYStyle,
        resut: &mut Vec<(usize, usize, String)>,
        put_defs_refs_here: &mut Vec<(usize, usize, &'static str, String)>,
    ) -> usize {
        put_defs_refs_here.push((
            start_x,
            start_y,
            "def",
            self.left_box.text_content.clone(),
        ));
        resut.push((start_x, start_y, self.left_box.text_content.clone()));
        resut.push((start_x + max_left_rule, start_y, " : ".to_string()));
        self.right_box.as_tty(
            start_x + max_left_rule + 3,
            start_y,
            dims_tty,
            resut,
            put_defs_refs_here,
        );
        return start_y + 0 + self.right_box_tty_hojd;
    }
    /*................................................*/
    pub fn as_svg(
        &mut self,
        start_x: usize,
        start_y: usize,
        max_left_rule: usize,
        dims_svg: &Dims,
        putdatarowshere: &mut Vec<String>,
    ) -> usize {
        putdatarowshere.push(Bocks::svg_text_out(
            "TEXTRULE",
            self.left_box.text_content.clone(),
            start_x as f32,
            start_y as f32,
            dims_svg,
        ));
        putdatarowshere.push(Bocks::svg_text_out(
            "TEXTRULE",
            ":".to_string(),
            (start_x + max_left_rule) as f32,
            start_y as f32,
            dims_svg,
        ));
        // and right side of rule
        self.right_box.as_svg(
            start_x + max_left_rule + 3,
            start_y,
            dims_svg,
            putdatarowshere,
        );
        return start_y + 0 + self.right_box_tty_hojd;
    }
}
