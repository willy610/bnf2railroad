//use bnf::BNF;
use ebnf::EBNF;
use std::rc::Rc;
use std::cell::RefCell;

use StOptional;
use StChoice;
use StIterOneOrMore;
use StIterZeroOrMore;
use TTYStyle;
use SVGStyle;
use Dims;
pub struct Grammarmother {
    pub kind: String,
    pub ebnf_kids: Vec<Rc<RefCell<EBNF>>>,
    pub dims: Dims,
}
impl Grammarmother {
    pub fn new() -> Grammarmother {
        Grammarmother {
            kind: "".to_string(),
            ebnf_kids: Vec::new(),
            dims: Dims {
                svg_style: SVGStyle {
                    scale_x: 8.0, // font size
                    scale_y: 12.0, // font size
                    line_dist: 3.0,
                    radie_lines: 3.0 * 8.0 / 2.0, // pixels // font size
                    rect_corner_radie: 10.0,
                },
                tty_style: TTYStyle {
                    the_opt: StOptional {
                        first_row: vec!["-[>", ">]-"],
                        second_row: vec![" [>", ">] "],
                    },
                    the_choice: StChoice {
                        first_row: vec!["-[>", ">]-"],
                        other_rows: vec![" [>", ">] "],
                        empty_rows: vec![" [", "] "],
                    },
                    the_iter_one_or_more: StIterOneOrMore {
                        first_row: vec!["-{>", ">}-"],
                        empty_rows: vec![" {", "} "],
                        back_row: vec![" {<", "<} "],
                    },
                    the_iter_zero_or_more: StIterZeroOrMore {
                        first_row: vec!["-{>-[", "]->}-"],
                        second_row: vec![" {>-{>", ">}-> "], // [>-{>- s ->}->]
                        empty_rows: vec!["    {", "}    "],
                        back_row: vec![" {<", "<} "],
                    },
                },
            },
        }
    }
    /*...............................................................*/
    pub fn set_kind(&mut self, kind: String) {
        self.kind = kind
    }
    /*...............................................................*/
    pub fn gen_svg_pre(
        &mut self,
        max_vidd: usize,
        total_hojd: usize,
        putdatarowshere: &mut Vec<String>,
    ) {
        putdatarowshere.push(format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\"
id=\"canvas\" width=\"{}\" height=\"{}\" preserveAspectRatio=\"xMidYMid\">
\n",
            max_vidd as f32 * self.dims.svg_style.scale_x,
            self.dims.svg_style.line_dist * (total_hojd + 1) as f32 *
                self.dims.svg_style.scale_y
        ));
        putdatarowshere.push("<defs><style type=\"text/css\">\n".to_string());
        putdatarowshere.push(
          "
.TEXT {font-size:12px;font-weight: normal;font-family: Lucida Console;stroke:none;fill:black;}
.TEXTRULE {font-size:12px;font-weight: normal;font-family: Lucida Console;stroke:none;fill:black;}
.TEXTBOLD {font-size:12px;font-weight: bold;font-family: Lucida Console;stroke:none;fill:black;}
.TEXTLITERAL {font-size:12px;font-weight: normal;font-family: Lucida Console;stroke:none;fill:black;}
.BOXID {fill:Khaki}
.BOXLITERAL {fill:lightgreen}
.BOXRESEREDWORD {fill:LightCyan}
.BOXGROUND {fill:black}
.PIL {fill:black}
.RAILSEQ {stroke:black; fill:none; stroke-width:1.5}
.RAILCHOICE {stroke:black; fill:none; stroke-width:1.5}
.RAILITER {stroke:black; fill:none; stroke-width:1.5}
.RAILOPT {stroke:black; fill:none; stroke-width:1.5}
.BOXOUTLINE {stroke:green; stroke-width:1.5; }
.TITEL {font-size:130%;font-weight: normal;font-family: Lucida Console;stroke:none;fill:black;}
          ".to_string());
        putdatarowshere.push("</style></defs>\n".to_string());
        putdatarowshere.push("<g>\n".to_string());
    }
    /*...............................................................*/
    pub fn gen_svg_post(&mut self, putdatarowshere: &mut Vec<String>) {
        // final
        putdatarowshere.push("</g></svg>\n".to_string());
    }
}
