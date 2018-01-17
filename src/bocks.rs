use std::rc::Rc;
use std::cell::RefCell;
use Dims;
use TTYStyle;
use EBNF;
#[derive(Debug)]
pub struct Bocks {
    pub kind: String,
    pub text_content: String,
    pub kids: Vec<Rc<RefCell<Bocks>>>,
    pub occurs: String,
    pub max_iter: String,
    pub tty_vidd: usize,
    pub tty_hojd: usize,
    pub literal_text_content_as_u8: u8, // just for dealing with ~ and genrule
}
//https://stackoverflow.com/questions/3062746/special-simple-random-number-generator
fn f_next_seed(old_seed: usize) -> usize {
    let a: usize = 1103515245;
    let m: usize = 2usize.pow(32); // 2^32 is 4294967296
    let c: usize = 12345;
    let next_seed = (a * old_seed + c) % m;
    next_seed
}
impl Bocks {
    pub fn new(kind: String, text_content: String) -> Bocks {
        Bocks {
            kind: kind,
            text_content: text_content,
            kids: Vec::new(),
            occurs: String::new(),
            max_iter: String::new(),
            tty_vidd: 0,
            tty_hojd: 0,
            literal_text_content_as_u8: 0,
        }
    }
    /*................................................*/
    pub fn calc_vidd(&mut self) -> usize {
        match self.kind.as_str() {
            "rule" => return 0 + self.text_content.len(),
            "sequence" | "group" => {
                let total_vidd = self.kids.iter().map(|x| x.borrow_mut().calc_vidd()).fold(
                    0,
                    |sum,
                     i| {
                        sum + i
                    },
                );
                self.tty_vidd = total_vidd;
                return self.tty_vidd;
            }
            "single_quoted" | "double_qouted" | "reserved_word" => {
                self.tty_vidd = 6 + self.text_content.len();
                return self.tty_vidd;
            }
            "identifier" => {
                self.tty_vidd = 4 + self.text_content.len();
                return self.tty_vidd;
            }
            "interval" => {
                // - 'a' -_~_- 'z' -
                // 1234567890123456789
                self.tty_vidd = 17;
                return self.tty_vidd;
            }
            "setdifference" => {
                // - identfirst - _-_ - identsecond -
                // 1234567890123456789
                let first_vidd = self.kids[0].borrow_mut().calc_vidd();
                let second_vidd = self.kids[1].borrow_mut().calc_vidd();
                self.tty_vidd = first_vidd + 5 + second_vidd;
                return self.tty_vidd;
            }
            "choice" => {
                let max_vidd = 0 +
                    self.kids.iter().map(|x| x.borrow_mut().calc_vidd()).fold(
                        0,
                        |max_sofar, i| if i > max_sofar { i } else { max_sofar },
                    );
                self.tty_vidd = 6 + max_vidd;
                return self.tty_vidd;
            }
            "iter_one_or_more" => {
                let total_vidd = self.kids.iter().map(|x| x.borrow_mut().calc_vidd()).fold(
                    0,
                    |sum,
                     i| {
                        sum + i
                    },
                );
                self.tty_vidd = total_vidd + 6;
                return self.tty_vidd;
            }
            "iter_with_list_separator" => {
                let total_vidd = self.kids.iter().map(|x| x.borrow_mut().calc_vidd()).fold(
                    0,
                    |sum,
                     i| {
                        sum + i
                    },
                );
                self.tty_vidd = total_vidd + 6;
                return self.tty_vidd;
            }
            "iter_zero_or_more" => {
                let total_vidd = self.kids.iter().map(|x| x.borrow_mut().calc_vidd()).fold(
                    0,
                    |sum,
                     i| {
                        sum + i
                    },
                );
                self.tty_vidd = total_vidd + 12;
                return self.tty_vidd;
            }
            "optional" => {
                let total_vidd = self.kids.iter().map(|x| x.borrow_mut().calc_vidd()).fold(
                    0,
                    |sum,
                     i| {
                        sum + i
                    },
                );
                self.tty_vidd = total_vidd + 6;
                return self.tty_vidd;
            }
            _ => panic!("Unknown for Bocks::calc_vidd {}", self.kind.as_str()),
        }
    }
    /*................................................*/
    pub fn calc_hojd(&mut self) -> usize {
        match self.kind.as_str() {
            // no kids
            "rule" => {
                self.tty_hojd = 1;
                return self.tty_hojd;
            }
            "sequence" |
            "optional" |
            "iter_one_or_more" |
            "iter_zero_or_more" |
            "iter_with_list_separator" |
            "group" => {
                let mut max_hojd = 0 +
                    self.kids.iter().map(|x| x.borrow_mut().calc_hojd()).fold(
                        0,
                        |max_sofar, i| if i > max_sofar { i } else { max_sofar },
                    );
                match self.kind.as_str() {
                    "sequence" => max_hojd += 0,
                    "optional" => max_hojd += 1,
                    "iter_one_or_more" => max_hojd += 1,
                    "iter_zero_or_more" => max_hojd += 2,
                    "iter_with_list_separator" => max_hojd += 1,
                    "group" => {}
                    _ => {}
                };
                self.tty_hojd = 0 + max_hojd;
                return self.tty_hojd;
            }
            "identifier" | "single_quoted" | "double_qouted" | "reserved_word" | "interval" |
            "setdifference" => {
                self.tty_hojd = 1;
                return self.tty_hojd;
            }
            "choice" => {
                let total_hojd = self.kids.iter().map(|x| x.borrow_mut().calc_hojd()).fold(
                    0,
                    |sum,
                     i| {
                        sum + i
                    },
                );
                self.tty_hojd = total_hojd + 0;
                return self.tty_hojd;
            }
            // => 1,
            //            "group" => 2,
            _ => panic!("Unknown for Bocks::calc_hojd {}", self.kind.as_str()),
        }
    }
    /*................................................*/
    pub fn as_tty(
        &mut self,
        at_x: usize,
        at_y: usize,
        tty_style: &TTYStyle,
        mut resut: &mut Vec<(usize, usize, String)>,
        mut defs_refs: &mut Vec<(usize, usize, &'static str, String)>,
    ) -> usize {
        let mut _local_x = at_x;
        let mut _local_y = at_y;
        // don't convert any thml reserved signs here but only in svg
        match self.kind.as_str() {
            "sequence" | "group" => {
                for i in 0..self.kids.len() {
                    let mut a_bocks = self.kids[i].borrow_mut();
                    _local_x = a_bocks.as_tty(_local_x, at_y, tty_style, resut, &mut defs_refs);
                }
                return _local_x;
            }
            "rule" => panic!("No rule for Bocks::as_tty "),
            "single_quoted" => {
                let str_res = format!("- '{}' -", self.text_content);
                resut.push((at_x, at_y, str_res.to_string()));
                return at_x + str_res.len();
            }
            "double_qouted" | "reserved_word" => {
                let str_res = format!("- \"{}\" -", self.text_content);
                resut.push((at_x, at_y, str_res.to_string()));
                return at_x + str_res.len();
            }
            "identifier" => {
                let str_res = format!("- {} -", self.text_content.clone());
                resut.push((at_x, at_y, str_res.to_string()));
                defs_refs.push((at_x + 2, at_y, "ref1", self.text_content.clone()));
                defs_refs.push((
                    at_x + 2 + self.text_content.len(),
                    at_y,
                    "ref2",
                    "</a>".to_string(),
                ));
                return at_x + str_res.len();
            }
            "interval" => {
                let inter_res = format!(
                    "- '{}' - ~ - '{}' -",
                    self.kids[0].borrow().text_content,
                    self.kids[1].borrow().text_content
                );
                resut.push((at_x, at_y, inter_res.to_string()));
                return at_x + inter_res.len();
            }
            "setdifference" => {
                let mut a_bocks = self.kids[0].borrow_mut();
                _local_x = a_bocks.as_tty(_local_x, at_y, tty_style, resut, &mut defs_refs);
                resut.push((_local_x, at_y, "- - -".to_string()));
                _local_x += 5;
                a_bocks = self.kids[1].borrow_mut();
                _local_x = a_bocks.as_tty(_local_x, at_y, tty_style, resut, &mut defs_refs);
                return _local_x;
            }
            "choice" => {
                //123
                //-[>----->]-
                // [>- s ->]
                let v = vec!["-"; self.tty_vidd - 6];
                let s: String = v.into_iter().collect();
                let str_one = format!(
                    "{}{}{}",
                    tty_style.the_choice.first_row[0],
                    s,
                    tty_style.the_choice.first_row[1]
                );
                let str_rest = format!(
                    "{}{}{}",
                    tty_style.the_choice.other_rows[0],
                    s,
                    tty_style.the_choice.other_rows[1]
                );
                _local_x += 3;
                for kid_row in 0..self.kids.len() {
                    if _local_y == at_y {
                        resut.push((at_x, _local_y, str_one.clone()));
                    } else {
                        resut.push((at_x, _local_y, str_rest.clone()));
                    }
                    let mut a_bocks = self.kids[kid_row].borrow_mut();
                    a_bocks.as_tty(_local_x, _local_y, tty_style, &mut resut, &mut defs_refs);
                    // advance to the next line.
                    let mut border_at_y = _local_y + 1;
                    // there might be rows with some margin spaces to the next kid tough
                    // advance to the next kid row
                    _local_y += a_bocks.tty_hojd;
                    //                    if kid_row < self.tty_hojd - 1 {
                    if kid_row < self.kids.len() - 1 {
                        // not the last kid
                        while border_at_y < _local_y {
                            resut.push((
                                at_x + 0,
                                border_at_y,
                                tty_style.the_choice.empty_rows[0].to_string(),
                            ));
                            resut.push((
                                at_x + self.tty_vidd - 2,
                                border_at_y,
                                tty_style.the_choice.empty_rows[1].to_string(),
                            ));
                            border_at_y += 1;
                        }
                    };
                }
                return at_x + self.tty_vidd;
            }
            "iter_with_list_separator" |
            "iter_one_or_more" => {
                //123
                //-{>- ssssssssss ->}-
                // {<-- separator --<}
                //                println!("self.tty_vidd={}", self.tty_vidd);
                //https://stackoverflow.com/questions/23430735/how-to-convert-vecchar-to-a-string
                let v = vec!["-"; self.tty_vidd - 6];
                let s: String = v.into_iter().collect();
                let str_one =
                    format!(
                    "{}{}{}",
                    tty_style.the_iter_one_or_more.first_row[0],
                    s,
                    tty_style.the_iter_one_or_more.first_row[1],
                );
                let str_back_row =
                    format!(
                    "{}{}{}",
                    tty_style.the_iter_one_or_more.back_row[0],
                    s,
                    tty_style.the_iter_one_or_more.back_row[1],
                );
                resut.push((at_x, at_y, str_one));
                let mut border_at_y = at_y;
                for index in 0..self.kids.len() {
                    let mut a_bocks = self.kids[index].borrow_mut();
                    a_bocks.as_tty(at_x + 3, _local_y, tty_style, &mut resut, &mut defs_refs);
                    _local_y += a_bocks.tty_hojd; //row space
                    border_at_y += 1;
                    while border_at_y < _local_y {
                        resut.push((
                            at_x + 0,
                            border_at_y,
                            tty_style.the_iter_one_or_more.empty_rows[0].to_string(),
                        ));
                        resut.push((
                            at_x + self.tty_vidd - 2,
                            border_at_y,
                            tty_style.the_iter_one_or_more.empty_rows[1].to_string(),
                        ));
                        border_at_y += 1;
                    }
                }
                // done. final row
                resut.push((at_x, _local_y, str_back_row));
                if self.kind.as_str() == "iter_with_list_separator" {
                    let str_res = format!("- '{}' -", self.text_content);
                    let right_adjusted_at_x = self.tty_vidd - 6 - 4 - self.text_content.len();
                    resut.push((at_x + right_adjusted_at_x, _local_y, str_res.to_string()));
                }
                return at_x + self.tty_vidd;
            }
            "iter_zero_or_more" => {
                //123456
                //-[>----------->]-
                // [>-{>- s ->}->]
                //    {<-----<}
                let v1 = vec!["-"; self.tty_vidd - (5 + 5)];
                let s1: String = v1.into_iter().collect();
                let str_one =
                    format!(
                    "{}{}{}",
                    tty_style.the_iter_zero_or_more.first_row[0],
                    s1,
                    tty_style.the_iter_zero_or_more.first_row[1],
                );

                let v2 = vec!["-"; self.tty_vidd - (6 + 6)];
                let s2: String = v2.into_iter().collect();
                let str_two =
                    format!(
                    "{}{}{}",
                    tty_style.the_iter_zero_or_more.second_row[0],
                    s2,
                    tty_style.the_iter_zero_or_more.second_row[1],
                );
                resut.push((at_x, at_y, str_one));
                resut.push((at_x, at_y + 1, str_two));
                _local_y += 1;
                let mut border_at_y = at_y;
                for index in 0..self.kids.len() {
                    let mut a_bocks = self.kids[index].borrow_mut();
                    a_bocks.as_tty(at_x + 6, _local_y, tty_style, &mut resut, &mut defs_refs);
                    _local_y += a_bocks.tty_hojd; //row space
                    border_at_y += 1;
                    while border_at_y < _local_y {
                        resut.push((
                            at_x,
                            border_at_y,
                            tty_style.the_iter_zero_or_more.empty_rows[0].to_string(),
                        ));
                        resut.push((
                            at_x + self.tty_vidd - 2 - 3,
                            border_at_y,
                            tty_style.the_iter_zero_or_more.empty_rows[1].to_string(),
                        ));
                        border_at_y += 1;
                    }
                }
                let v3 = vec!["-"; self.tty_vidd - (6 + 6)];
                let s3: String = v3.into_iter().collect();

                let str_three = format!(
                    "{}{}{}",
                    tty_style.the_iter_zero_or_more.back_row[0],
                    s3,
                    tty_style.the_iter_zero_or_more.back_row[1]
                );
                resut.push((at_x + 3, _local_y - 0, str_three));
                return at_x + self.tty_vidd;
            }
            "optional" => {
                //123
                //-[>----->]-
                // [>- s ->]
                let v = vec!["-"; self.tty_vidd - 6];
                let s: String = v.into_iter().collect();
                //                let str_one = format!("-[>{}>]-", s);
                let str_one =
                    format!(
                    "{}{}{}",
                    tty_style.the_opt.first_row[0],
                    s,
                    tty_style.the_opt.first_row[1],
                );
                //                let str_two = format!(" [>{}>] ", s);
                let str_two =
                    format!(
                    "{}{}{}",
                    tty_style.the_opt.second_row[0],
                    s,
                    tty_style.the_opt.second_row[1],

                                );
                resut.push((_local_x, at_y, str_one));
                resut.push((_local_x, at_y + 1, str_two));
                _local_x += 3;
                for i in 0..self.kids.len() {
                    let mut a_bocks = self.kids[i].borrow_mut();
                    _local_x =
                        a_bocks.as_tty(_local_x, at_y + 1, tty_style, &mut resut, &mut defs_refs);
                    _local_x += 0; //right space
                }
                return at_x + self.tty_vidd;
            }
            _ => panic!("Unknown rule for Bocks::as_tty {}", self.kind.as_str()),
        }
    }
    /*................................................*/
    pub fn _dump(&mut self) {
        println!(
            "self.kind,self.tty_vidd,self.tty_hojd={} {} {}",
            self.kind,
            self.tty_vidd,
            self.tty_hojd
        );
        for i in 0..self.kids.len() {
            let mut a_bocks = self.kids[i].borrow_mut();
            a_bocks._dump();
        }
    }
    /*................................................*/
    pub fn as_svg(
        &mut self,
        at_x: usize,
        at_y: usize,
        dims_svg: &Dims,
        mut putdatarowshere: &mut Vec<String>,
    ) -> usize {
        let mut _local_x = at_x;
        let mut _local_y = at_y;
        match self.kind.as_str() {
            "rule" => panic!("No rule for Bocks::as_svg "),
            "sequence" | "group" => {
                for i in 0..self.kids.len() {
                    let mut a_bocks = self.kids[i].borrow_mut();
                    putdatarowshere.push(Bocks::svg_line_out(
                        "RAILSEQ",
                        _local_x as f32,
                        at_y as f32,
                        (_local_x + a_bocks.tty_vidd) as f32,
                        at_y as f32,
                        dims_svg,
                    ));
                    _local_x = a_bocks.as_svg(_local_x, at_y, dims_svg, putdatarowshere);
                }
                return at_x + self.tty_vidd;
            }
            "identifier" | "single_quoted" | "double_qouted" | "reserved_word" => {
                putdatarowshere.push(Bocks::svg_text_and_box(
                    self.kind.clone(),
                    self.text_content.clone(),
                    _local_x as f32,
                    at_y as f32,
                    dims_svg,
                ));
                return at_x + self.tty_vidd;
            }
            "interval" => {
                // - 'a' - _~_ - 'z' -
                // 1234567890123456789
                // self.tty_vidd = 7 + 5 + 7;

                let left = self.kids[0].borrow();
                putdatarowshere.push(Bocks::svg_text_and_box(
                    left.kind.clone(),
                    left.text_content.clone(),
                    _local_x as f32,
                    at_y as f32,
                    dims_svg,
                ));
                putdatarowshere.push(Bocks::svg_text_and_box(
                    "reserved_word".to_string(),
                    self.text_content.clone(),
                    (_local_x + 7) as f32,
                    at_y as f32,
                    dims_svg,
                ));
                let right = self.kids[1].borrow();
                putdatarowshere.push(Bocks::svg_text_and_box(
                    right.kind.clone(),
                    right.text_content.clone(),
                    (_local_x + 7 + 5) as f32,
                    at_y as f32,
                    dims_svg,
                ));
                return at_x + self.tty_vidd;
            }
            "setdifference" => {
                let mut left = self.kids[0].borrow_mut();
                _local_x = left.as_svg(_local_x, at_y, dims_svg, putdatarowshere);

                putdatarowshere.push(Bocks::svg_text_and_box(
                    "reserved_word".to_string(),
                    self.text_content.clone(),
                    (_local_x + 0) as f32,
                    at_y as f32,
                    dims_svg,
                ));
                _local_x += 5;
                let mut right = self.kids[1].borrow_mut();
                _local_x = right.as_svg(_local_x, at_y, dims_svg, putdatarowshere);
                return at_x + self.tty_vidd;
            }
            "choice" => {
                // horis line full width
                putdatarowshere.push(Bocks::svg_line_out(
                    "RAILSEQ",
                    _local_x as f32,
                    at_y as f32,
                    (_local_x + self.tty_vidd) as f32,
                    at_y as f32,
                    dims_svg,
                ));
                for i in 0..self.kids.len() {
                    let mut a_bocks = self.kids[i].borrow_mut();
                    // all but last draw arrows left down an right up
                    if i != self.kids.len() - 1 {
                        putdatarowshere.push(Bocks::svg_pil_out(
                            "PIL",
                            _local_x as f32 + 1.5,
                            _local_y as f32 + (a_bocks.tty_hojd as f32) / 2.0,
                            0,
                            dims_svg,
                        ));
                        putdatarowshere.push(Bocks::svg_pil_out(
                            "PIL",
                            (_local_x + self.tty_vidd) as f32 - 1.5,
                            _local_y as f32 + (a_bocks.tty_hojd as f32) / 2.0,
                            180,
                            dims_svg,
                        ));
                    }
                    // horis line more narrow ( not first and not last)
                    // all but first a shorter horiz line and krok left and right
                    if i != 0 {
                        putdatarowshere.push(Bocks::svg_line_out(
                            "RAILSEQ",
                            _local_x as f32 + 3.0,
                            _local_y as f32,
                            (_local_x + self.tty_vidd) as f32 - 3.0,
                            _local_y as f32,
                            dims_svg,
                        ));
                        // and krokar left and rigth
                        putdatarowshere.push(Bocks::svg_krok(
                            "RAILSEQ",
                            _local_x as f32 + 1.5,
                            _local_y as f32,
                            270,
                            dims_svg,
                        ));
                        putdatarowshere.push(Bocks::svg_krok(
                            "RAILSEQ",
                            (_local_x + self.tty_vidd) as f32 - 1.5,
                            _local_y as f32,
                            180,
                            dims_svg,
                        ));
                    }
                    // horis last. now we nice height.
                    // draw left and right svg_optional_left svg_optional_right
                    if i == self.kids.len() - 1 {
                        /*                        putdatarowshere.push(Bocks::svg_line_out(
                            "RAILSEQ",
                            _local_x as f32 + 3.0,
                            (_local_y + a_bocks.tty_hojd) as f32,
                            (_local_x + self.tty_vidd) as f32 - 3.0,
                            (_local_y + a_bocks.tty_hojd) as f32,
                            dims_svg,
                        ));*/
                        putdatarowshere.push(Bocks::svg_optional_left(
                            "RAILOPT",
                            at_x as f32,
                            at_y as f32,
                            (_local_y - at_y) as f32,
                            dims_svg,
                        ));
                        putdatarowshere.push(Bocks::svg_optional_right(
                            "RAILOPT",
                            (at_x + self.tty_vidd) as f32,
                            at_y as f32,
                            (_local_y - at_y) as f32,
                            dims_svg,
                        ));
                    }
                    a_bocks.as_svg(_local_x + 3, _local_y, dims_svg, putdatarowshere);
                    _local_y += a_bocks.tty_hojd; //row space
                }
                return at_x + self.tty_vidd;
            }
            "iter_with_list_separator" |
            "iter_one_or_more" => {
                putdatarowshere.push(Bocks::svg_line_out(
                    "RAILSEQ",
                    at_x as f32,
                    at_y as f32,
                    (at_x + self.tty_vidd) as f32,
                    at_y as f32,
                    dims_svg,
                ));

                putdatarowshere.push(Bocks::svg_iter_left(
                    "RAILOPT",
                    at_x as f32,
                    at_y as f32,
                    (self.tty_hojd - 1) as f32,
                    dims_svg,
                ));
                putdatarowshere.push(Bocks::svg_iter_right(
                    "RAILOPT",
                    (at_x + self.tty_vidd) as f32,
                    at_y as f32,
                    (self.tty_hojd - 1) as f32,
                    dims_svg,
                ));

                putdatarowshere.push(Bocks::svg_pil_out(
                    "PIL",
                    at_x as f32 + 1.5,
                    (at_y + self.tty_hojd - 1) as f32 - 0.5,
                    180,
                    dims_svg,
                ));

                putdatarowshere.push(Bocks::svg_pil_out(
                    "PIL",
                    (at_x + self.tty_vidd) as f32 - 1.5,
                    (at_y + self.tty_hojd - 1) as f32 - 0.5,
                    0,
                    dims_svg,
                ));

                _local_x += 3;
                for i in 0..self.kids.len() {
                    let mut a_bocks = self.kids[i].borrow_mut();
                    a_bocks.as_svg(_local_x, _local_y, dims_svg, putdatarowshere);
                    //                    _local_y += 1;
                    _local_y += a_bocks.tty_hojd; //row space
                }
                putdatarowshere.push(Bocks::svg_line_out(
                    "RAILSEQ",
                    _local_x as f32,
                    _local_y as f32,
                    (_local_x + self.tty_vidd - 6) as f32,
                    _local_y as f32,
                    dims_svg,
                ));
                if self.kind.as_str() == "iter_with_list_separator" {
                    let right_adjusted_at_x = self.tty_vidd - 6 - 4 - self.text_content.len();
                    putdatarowshere.push(Bocks::svg_text_and_box(
                        self.kind.clone(),
                        self.text_content.clone(),
                        //                    _local_x as f32,
                        (at_x + right_adjusted_at_x) as f32,
                        _local_y as f32,
                        dims_svg,
                    ));

                }
                return at_x + self.tty_vidd;
            }
            "iter_zero_or_more" => {
                putdatarowshere.push(Bocks::svg_line_out(
                    "RAILSEQ",
                    _local_x as f32,
                    _local_y as f32,
                    (_local_x + self.tty_vidd) as f32,
                    _local_y as f32,
                    dims_svg,
                ));
                putdatarowshere.push(Bocks::svg_optional_left(
                    "RAILOPT",
                    _local_x as f32 + 0.0,
                    at_y as f32,
                    1.0,
                    dims_svg,
                ));
                putdatarowshere.push(Bocks::svg_pil_out(
                    "PIL",
                    _local_x as f32 + 1.5,
                    _local_y as f32 + 0.5,
                    0,
                    dims_svg,
                ));

                putdatarowshere.push(Bocks::svg_pil_out(
                    "PIL",
                    (_local_x + self.tty_vidd) as f32 - 1.5,
                    _local_y as f32 + 0.5,
                    180,
                    dims_svg,
                ));

                putdatarowshere.push(Bocks::svg_optional_right(
                    "RAILOPT",
                    (_local_x + self.tty_vidd) as f32 - 0.0,
                    at_y as f32,
                    1.0,
                    dims_svg,
                ));
                ////////////////////////
                // noe advance one row and srhrink width
                // horis line full width
                _local_x += 3;
                _local_y += 1;
                let _local_shrinked_vidd = self.tty_vidd - 6;

                putdatarowshere.push(Bocks::svg_line_out(
                    "RAILSEQ",
                    _local_x as f32,
                    _local_y as f32,
                    (_local_x + _local_shrinked_vidd) as f32,
                    _local_y as f32,
                    dims_svg,
                ));

                putdatarowshere.push(Bocks::svg_iter_left(
                    "RAILOPT",
                    _local_x as f32,
                    _local_y as f32,
                    (self.tty_hojd - 2) as f32,
                    dims_svg,
                ));
                putdatarowshere.push(Bocks::svg_iter_right(
                    "RAILOPT",
                    (_local_x + _local_shrinked_vidd) as f32,
                    _local_y as f32,
                    (self.tty_hojd - 2) as f32,
                    dims_svg,
                ));
                for i in 0..self.kids.len() {
                    let mut a_bocks = self.kids[i].borrow_mut();
                    a_bocks.as_svg(_local_x + 3, _local_y, dims_svg, putdatarowshere);
                    _local_y += a_bocks.tty_hojd; //row space
                }
                putdatarowshere.push(Bocks::svg_pil_out(
                    "PIL",
                    _local_x as f32 + 1.5,
                    _local_y as f32 - 0.5,
                    180,
                    dims_svg,
                ));
                putdatarowshere.push(Bocks::svg_pil_out(
                    "PIL",
                    (_local_x + _local_shrinked_vidd) as f32 - 1.5,
                    _local_y as f32 - 0.5,
                    0,
                    dims_svg,
                ));
                // middle back arrow
                putdatarowshere.push(Bocks::svg_pil_out(
                    "PIL",
                    (_local_x + _local_shrinked_vidd / 2) as f32,
                    _local_y as f32,
                    90,
                    dims_svg,
                ));
                // horis last
                putdatarowshere.push(Bocks::svg_line_out(
                    "RAILSEQ",
                    _local_x as f32 + 3.0,
                    _local_y as f32,
                    (_local_x + _local_shrinked_vidd) as f32 - 3.0,
                    _local_y as f32,
                    dims_svg,
                ));
                return at_x + self.tty_vidd;
            }
            "optional" => {
                putdatarowshere.push(Bocks::svg_optional_left(
                    "RAILOPT",
                    _local_x as f32,
                    at_y as f32,
                    1.0,
                    dims_svg,
                ));
                putdatarowshere.push(Bocks::svg_pil_out(
                    "PIL",
                    _local_x as f32 + 1.5,
                    at_y as f32 + 0.5 as f32,
                    0,
                    dims_svg,
                ));
                putdatarowshere.push(Bocks::svg_optional_right(
                    "RAILOPT",
                    (_local_x + self.tty_vidd) as f32,
                    at_y as f32,
                    1.0,
                    dims_svg,
                ));
                putdatarowshere.push(Bocks::svg_pil_out(
                    "PIL",
                    (_local_x + self.tty_vidd) as f32 - 1.5,
                    at_y as f32 + 0.5 as f32,
                    180,
                    dims_svg,
                ));
                _local_x += 3;
                for i in 0..self.kids.len() {
                    let mut a_bocks = self.kids[i].borrow_mut();
                    putdatarowshere.push(Bocks::svg_line_out(
                        "RAILSEQ",
                        _local_x as f32,
                        (at_y + 1) as f32,
                        (_local_x + self.tty_vidd - 6) as f32,
                        (at_y + 1) as f32,
                        dims_svg,
                    ));
                    _local_x = a_bocks.as_svg(_local_x, at_y + 1, dims_svg, putdatarowshere);
                    _local_x += 0; //right space
                }
                return at_x + self.tty_vidd;
            }
            _ => panic!("Unknown rule for Bocks::as_svg {}", self.kind.as_str()),
        }
    }
    /*................. STATIC FUNCTIONS ..................*/
    pub fn svg_text_out(
        cls: &'static str,
        txt: String,
        at_x: f32,
        at_y: f32,
        dims_svg: &Dims,
    ) -> String {
        format!(
            "<text class=\"{}\" x=\"{}\" y=\"{}\">{}</text>\n",
            cls,
            at_x * dims_svg.svg_style.scale_x, /*+ 2.0 * dims_svg.svg_style.scale_x*/
            at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist +
                dims_svg.svg_style.scale_y / 4.0,
            txt
        )
    }
    /*..................................................*/
    pub fn svg_line_out(
        cls: &'static str,
        at_x: f32,
        at_y: f32,
        end_x: f32,
        end_y: f32,
        dims_svg: &Dims,
    ) -> String {
        format!(
            "<line class=\"{}\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"></line>\n",
            cls,
            at_x * dims_svg.svg_style.scale_x,
            at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist,
            end_x * dims_svg.svg_style.scale_x,
            end_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist
        )
    }
    /*..................................................*/
    pub fn svg_optional_left(
        cls: &'static str,
        at_x: f32,
        at_y: f32,
        hojd: f32,
        dims_svg: &Dims,
    ) -> String {
        format!(
            "<path class=\"{}\" d=\"M {},{} C {},{} {},{} {},{} L {},{} C {},{} {},{} {},{}\"></path>\n",
            cls,
            0.0 + at_x * dims_svg.svg_style.scale_x,
            0.0 + at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // M

            dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            0.0 + at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // Corner

            dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            0.0 + at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // Corner

            dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            dims_svg.svg_style.radie_lines +
                at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // end C

            dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            -dims_svg.svg_style.radie_lines +
                (at_y + hojd) * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // L

            dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            (at_y + hojd) * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // Corner

            dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            (at_y + hojd) * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // Corner

            2.0 * dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            (at_y + hojd) * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist // rigth end C
        )
    }
    /*..................................................*/
    pub fn svg_iter_left(
        cls: &'static str,
        at_x: f32,
        at_y: f32,
        hojd: f32,
        dims_svg: &Dims,
    ) -> String {
        format!(
            "<path class=\"{}\" d=\"M {},{} C {},{} {},{} {},{} L {},{} C {},{} {},{} {},{}\"></path>\n",
            cls,
            2.0 * dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            0.0 + at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // M

            dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            0.0 + at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // Corner

            dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            0.0 + at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // Corner

            dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            dims_svg.svg_style.radie_lines +
                at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // end C

            dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            -dims_svg.svg_style.radie_lines +
                (at_y + hojd) * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // L

            dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            (at_y + hojd) * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // Corner

            dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            (at_y + hojd) * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // Corner

            2.0 * dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            (at_y + hojd) * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist // rigth end C
        )
    }
    /*..................................................*/
    pub fn svg_optional_right(
        cls: &'static str,
        at_x: f32,
        at_y: f32,
        hojd: f32,
        dims_svg: &Dims,
    ) -> String {
        format!(
            "<path class=\"{}\" d=\"M {},{} C {},{} {},{} {},{} L {},{} C {},{} {},{} {},{}\"></path>\n",
            cls,
            0.0 + at_x * dims_svg.svg_style.scale_x,
            0.0 + at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // M

            -dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            0.0 + at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // Corner

            -dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            0.0 + at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // Corner

            -dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            dims_svg.svg_style.radie_lines +
                at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // end C

            -dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            -dims_svg.svg_style.radie_lines +
                (at_y + hojd) * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // L

            -dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            (at_y + hojd) * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // Corner

            -dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            (at_y + hojd) * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // Corner

            -2.0 * dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            (at_y + hojd) * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist // rigth end C
        )
    }
    /*..................................................*/
    pub fn svg_iter_right(
        cls: &'static str,
        at_x: f32,
        at_y: f32,
        hojd: f32,
        dims_svg: &Dims,
    ) -> String {
        format!(
            "<path class=\"{}\" d=\"M {},{} C {},{} {},{} {},{} L {},{} C {},{} {},{} {},{}\"></path>\n",
            cls,
            -2.0 * dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            0.0 + at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // M

            -dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            0.0 + at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // Corner

            -dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            0.0 + at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // Corner

            -dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            dims_svg.svg_style.radie_lines +
                at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // end C

            -dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            -dims_svg.svg_style.radie_lines +
                (at_y + hojd) * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // L

            -dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            (at_y + hojd) * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // Corner

            -dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            (at_y + hojd) * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // Corner

            -2.0 * dims_svg.svg_style.radie_lines + at_x * dims_svg.svg_style.scale_x,
            (at_y + hojd) * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist // rigth end C
        )
    }
    /*..................................................*/
    pub fn svg_krok(
        cls: &'static str,
        at_x: f32,
        at_y: f32,
        vinkel: usize,
        dims_svg: &Dims,
    ) -> String {
        //<path class="RAILSEQ" d="M 144,0 C 0,0 0,0 0,144" transform="translate(80,108) rotate(180)"></path>
        format!(
            "<path class=\"{}\" d=\"M {},{} C {},{} {},{} {},{}\" transform=\"translate({},{}) rotate({})\"></path>\n",
            cls,
            dims_svg.svg_style.radie_lines,
            0,
            0,
            0,
            0,
            0,
            0,
            dims_svg.svg_style.radie_lines,
            at_x * dims_svg.svg_style.scale_x,
            at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist, // M
            vinkel
        )
    }
    /*..................................................*/
    pub fn svg_pil_out(
        cls: &'static str,
        at_x: f32,
        at_y: f32,
        vinkel: usize,
        dims_svg: &Dims,
    ) -> String {
        format!(
            "<path class=\"{}\" d=\"M-7,-8 L0,8 7,-8 0,0 -7,-8\" transform=\"translate({},{}) rotate({}) scale({},{})\"></path>",
            cls,
            at_x * dims_svg.svg_style.scale_x, // translate
            at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist,
            vinkel, //vinkel 0 is right down
            0.8,
            0.8
        )
    }
    /*..................................................*/
    pub fn svg_text_and_box(
        kind: String,
        txt: String,
        at_x: f32,
        at_y: f32,
        dims_svg: &Dims,
    ) -> String {
        let txt_cls;
        let back_rect_cls;
        let forg_rect_cls;
        let local_txt: String;
        let rect_corner: f32;
        // rect class, round corners
        // text padding ' or " or nothing
        if kind == "identifier" {
            txt_cls = "TEXT";
            forg_rect_cls = "BOXID";
            back_rect_cls = "BOXGROUND";
            local_txt = format!("{}", txt);
            rect_corner = 0.0;
        } else if kind == "single_quoted" || kind == "iter_with_list_separator" {
            txt_cls = "TEXT";
            forg_rect_cls = "BOXLITERAL";
            back_rect_cls = "BOXGROUND";
            local_txt = format!("'{}'", txt);
            rect_corner = dims_svg.svg_style.rect_corner_radie;
        } else if kind == "reserved_word" {
            txt_cls = "TEXT";
            forg_rect_cls = "BOXRESEREDWORD";
            back_rect_cls = "BOXGROUND";
            local_txt = format!("{}", txt);
            rect_corner = dims_svg.svg_style.rect_corner_radie;
        } else {
            txt_cls = "TEXT";
            forg_rect_cls = "BOXLITERAL";
            back_rect_cls = "BOXGROUND";
            local_txt = format!("\"{}\"", txt);
            rect_corner = dims_svg.svg_style.rect_corner_radie;
        }
        let vidd_f: f32 = local_txt.len() as f32 * dims_svg.svg_style.scale_x +
            2.0 * dims_svg.svg_style.scale_x;
        let txt_pos = (
            at_x * dims_svg.svg_style.scale_x + 2.0 * dims_svg.svg_style.scale_x,
            at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist +
                dims_svg.svg_style.scale_y / 4.0,
        );
        let rect_pos = (
            at_x * dims_svg.svg_style.scale_x + 1.0 * dims_svg.svg_style.scale_x,
            at_y * dims_svg.svg_style.scale_y * dims_svg.svg_style.line_dist -
                1.1 * dims_svg.svg_style.scale_y,
            vidd_f,
            2.0 * dims_svg.svg_style.scale_y,
        );
        let mut escaped_txt = local_txt.replace("&", "&#38;");
        escaped_txt = escaped_txt.replace("<", "&#60;");
        format!(
            "<rect class=\"{}\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\" ry=\"{}\"></rect>\n
<rect class=\"{}\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\" ry=\"{}\"></rect>\n
<text class=\"{}\" x=\"{}\" y=\"{}\">{}</text>\n\n",
            // backgrounf
            back_rect_cls,
            rect_pos.0 + dims_svg.svg_style.scale_x / 10.0,
            rect_pos.1 + 1.0 * dims_svg.svg_style.scale_y / 10.0,
            vidd_f, //width
            2.0 * dims_svg.svg_style.scale_y, // height
            rect_corner,
            rect_corner, // rx,ry
            // forground
            forg_rect_cls,
            rect_pos.0,
            rect_pos.1,
            vidd_f, //width
            2.0 * dims_svg.svg_style.scale_y, // height
            rect_corner,
            rect_corner, // rx,ry
            txt_cls,
            txt_pos.0,
            txt_pos.1,
            escaped_txt
        )
    }
    /*................................................*/
    pub fn gen_from_box(
        &self,
        putgentexthere: &mut String,
        the_ebnf: &EBNF,
        seed: usize,
        mut steps_remaining: usize,
    ) {
        let mut next_seed = f_next_seed(seed);
        if steps_remaining > 0 {
            steps_remaining = steps_remaining - 1;
        }
//        println!("{} self.kind={}",steps_remaining, self.kind);
        if steps_remaining == 0
        {
          println!("Maxsteps reached! (Result incomplet)");
          return ;
        }
        match self.kind.as_str() {
            "sequence" => {
                for i in 0..self.kids.len() {
                    let a_bocks = self.kids[i].borrow();
                    next_seed = f_next_seed(next_seed);
                    a_bocks.gen_from_box(putgentexthere, the_ebnf, next_seed, steps_remaining)
                }
            }
            "double_qouted" | "reserved_word" => {
                putgentexthere.push(' ');
                for a_char in self.text_content.chars() {
                    putgentexthere.push(a_char)
                }
                putgentexthere.push(' ');
            }
            "single_quoted" => {
                for a_char in self.text_content.chars() {
                    putgentexthere.push(a_char)
                }
            }
            "identifier" => {
                let the_ident = self.text_content.as_str();
                the_ebnf.find_rule_by_name(
                    the_ident,
                    putgentexthere,
                    next_seed, // seed start
                    steps_remaining,
                );
            }
            "choice" => {
                // select one in the choice
                let the_choice_index = seed % self.kids.len();
                let a_choice = self.kids[the_choice_index].borrow();
                a_choice.gen_from_box(putgentexthere, the_ebnf, next_seed, steps_remaining);
            }
            "optional" => {
                if seed % 2 == 0 {
                    for i in 0..self.kids.len() {
                        let a_bocks = self.kids[i].borrow();
                        next_seed = f_next_seed(next_seed);
                        a_bocks.gen_from_box(putgentexthere, the_ebnf, next_seed, steps_remaining)
                    }
                }
            }
            "group" => {
                for i in 0..self.kids.len() {
                    let a_bocks = self.kids[i].borrow();
                    next_seed = f_next_seed(next_seed);
                    a_bocks.gen_from_box(putgentexthere, the_ebnf, next_seed, steps_remaining)
                }
            }
            "iter_one_or_more" => {
                // iter [1,3] depending in seed
                let mut end_index = 1 + (next_seed % 3);
                if steps_remaining == 0 {
                    end_index = 1;
                }
                for _nr_step in 0..end_index {
                    for i in 0..self.kids.len() {
                        let a_bocks = self.kids[i].borrow();
                        next_seed = f_next_seed(next_seed);
                        a_bocks.gen_from_box(putgentexthere, the_ebnf, next_seed, steps_remaining)
                    }
                }
            }
            "iter_zero_or_more" => {
                // iter [1,3] depending in seed
                if seed % 2 == 0 && steps_remaining > 0 {
                    let mut end_index = 1 + (next_seed % 3);
                    if steps_remaining == 0 {
                        end_index = 1;
                    }

                    for _nr_step in 0..end_index {
                        for i in 0..self.kids.len() {
                            let a_bocks = self.kids[i].borrow();
                            next_seed = f_next_seed(next_seed);
                            a_bocks.gen_from_box(
                                putgentexthere,
                                the_ebnf,
                                next_seed,
                                steps_remaining,
                            )
                        }
                    }
                }
            }
            "iter_with_list_separator" => {
                let mut end_index = 1 + (next_seed % 3);
                if steps_remaining == 0 {
                    end_index = 1;
                }
                for _nr_step in 0..end_index {
                    for i in 0..self.kids.len() {
                        let a_bocks = self.kids[i].borrow();
                        next_seed = f_next_seed(next_seed);
                        a_bocks.gen_from_box(putgentexthere, the_ebnf, next_seed, steps_remaining);
                    }
                    if _nr_step < end_index - 1 {
                        putgentexthere.push(self.text_content.chars().next().unwrap());
                    }
                }
            }
            "interval" => {
                let left_value: u8 = self.kids[0].borrow().literal_text_content_as_u8;
                let right_value: u8 = self.kids[1].borrow().literal_text_content_as_u8;
                let random_value: u8 = left_value +
                    (next_seed % (right_value - left_value) as usize) as u8;
                let ret: char = random_value as char;
                putgentexthere.push(ret);
            }
            "setdifference"=>{
              self.kids[0].borrow().gen_from_box(putgentexthere, the_ebnf, next_seed, steps_remaining);
              putgentexthere.push('-');
              self.kids[1].borrow().gen_from_box(putgentexthere, the_ebnf, next_seed, steps_remaining);
            }
            _ => println!("kind not known={}", self.kind),
        }
    }
}
