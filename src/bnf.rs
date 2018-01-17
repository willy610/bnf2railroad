
use bocks::Bocks;
use rule::Rule;
use std::rc::Rc;
use std::cell::RefCell;
use Dims;
use TTYStyle;
use reswords::Reswords;
use refsdefs;
use refsdefs::RefsDefs;

use std::cmp::Ordering;

pub struct BNF {
    pub kind: String,
    pub grammer_file_name: String,
    pub the_word_list: Rc<RefCell<Reswords>>,
    pub the_grammer_as_rules: Vec<u8>,
    pub at: usize,
    pub ch: u8,
    pub max_left_rule: usize,
    // se MYRUST/testclasses4 och kanske 5 foer dubbellinked refs
    pub all_rule_objs: Vec<Rc<RefCell<Rule>>>,
    pub all_rows: Vec<Vec<char>>,
    pub def_ref_check_results: Rc<RefCell<RefsDefs>>,
}

impl BNF {
    pub fn new(
        grammer_file_name: String,
        the_grammer_as_rules: String,
        the_word_list: Rc<RefCell<Reswords>>,
        def_ref_check_results: Rc<RefCell<RefsDefs>>,
        kind: String,
    ) -> BNF {
        BNF {
            grammer_file_name: grammer_file_name,
            the_grammer_as_rules: Vec::from(the_grammer_as_rules.as_bytes()),
            at: 0,
            ch: 0,
            max_left_rule: 0,
            all_rule_objs: Vec::new(),
            all_rows: Vec::new(),
            the_word_list: the_word_list,
            def_ref_check_results: def_ref_check_results,
            kind: kind,
        }
    }
    /*................................................*/
    pub fn parse_grammer(&mut self) {
        self.at = 0;
        self.ch = 0;
        self.next(128); // nothing
        self.grammer();
    }
    /*................................................*/
    pub fn calc(&mut self, start_x: usize, start_y: usize, titel_vidd: usize) -> (usize, usize) {
        for i in 0..self.all_rule_objs.len() {
            let mut a_rule = self.all_rule_objs[i].borrow_mut();
            a_rule.calc_hojder();
            a_rule.calc_vidder();
        }
        // https://stackoverflow.com/questions/36368843/
        // rust-filtering-and-mapping-an-iterator-of-results
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html
        self.max_left_rule = 0 +
            self.all_rule_objs
                .iter()
                .map(|x| x.borrow_mut().left_box_tty_vidd)
                .fold(0, |max_sofar, i| if i > max_sofar { i } else { max_sofar });
        let mut max_right_rule: usize = 0 +
            self.all_rule_objs
                .iter()
                .map(|x| x.borrow_mut().right_box_tty_vidd)
                .fold(0, |max_sofar, i| if i > max_sofar { i } else { max_sofar });
        if max_right_rule < titel_vidd {
            max_right_rule = titel_vidd;
        }
        // total hojd
        let total_hojd = 0 +
            self.all_rule_objs
            .iter()
            .map(|x| 1 + x.borrow_mut().right_box_tty_hojd) // one extra row between rules
            .fold(0, |sum, i| sum + i);
        // now produce the print area
        // now dump
        /*
        for i in 0..self.all_rule_objs.len() {
            let mut a_rule = self.all_rule_objs[i].borrow_mut();
            a_rule._dump();
            a_rule._dump();
        }
        */
        // alloc write area and return sizes
        //        println!("BNF::calc() {},{}"
        //        ,start_x + self.max_left_rule + 3 + max_right_rule
        //        ,start_y + total_hojd);
        self.all_rows = vec![
            vec![' '; (start_x + self.max_left_rule + 3 + max_right_rule) as usize];
            (start_y + total_hojd) as usize
        ];
        //        println!("BNF::calc({},{}) ",start_y , total_hojd);
        (
            (start_x + self.max_left_rule + 3 + max_right_rule) as usize,
            (total_hojd) as usize,
        )
    }
    /*................................................*/
    pub fn as_svg(
        &mut self,
        par_start_x: usize,
        par_start_y: usize,
        dims_svg: &Dims,
        mut putdatarowshere: &mut Vec<String>,
    ) -> usize {
        let mut start_y: usize = par_start_y;
        // collect all rules
        let the_filname = format!("Grammar filename: {}", self.grammer_file_name);
        putdatarowshere.push(Bocks::svg_text_out(
            "TITEL",
            the_filname,
            par_start_x as f32,
            (par_start_y - 2) as f32,
            dims_svg,
        ));
        // for each rule
        for i in 0..self.all_rule_objs.len() {
            let mut a_rule = self.all_rule_objs[i].borrow_mut();
            start_y = a_rule.as_svg(
                par_start_x,
                start_y,
                self.max_left_rule,
                dims_svg,
                &mut putdatarowshere,
            );
            start_y += 1; //space between each rule
        }
        return start_y;
    }
    /*................................................*/
    pub fn as_tty(
        &mut self,
        par_start_x: usize,
        par_start_y: usize,
        dims_tty: &TTYStyle,

        putdatarowshere: &mut Vec<String>,
    ) {
        let mut start_y: usize = par_start_y;
        let mut resut: Vec<(usize, usize, String)> = Vec::new();
        let mut put_defs_refs_here: Vec<(usize, usize, &'static str, String)> = Vec::new();

        // collect filanem
        resut.push((
            par_start_x,
            par_start_y - 2,
            format!("Grammar filename: {}", self.grammer_file_name),
        ));
        // collect all rules
        // for each rule
        for i in 0..self.all_rule_objs.len() {
            let mut a_rule = self.all_rule_objs[i].borrow_mut();
            start_y = a_rule.as_tty(
                par_start_x,
                start_y,
                self.max_left_rule,
                dims_tty,
                &mut resut,
                &mut put_defs_refs_here,
            );
            start_y += 1; //space
        }
        for texten in resut {
            let x = texten.0;
            let y = texten.1;
            let txt = texten.2.as_bytes();
            let len = txt.len();
            for i in 0..len {
                self.all_rows[y][x + i] = txt[i] as char;
            }
        }
        // https://stackoverflow.com/questions/23430735/how-to-convert-vecchar-to-a-string
        // return results
        // merge in defs and refs
        //        println!("self.kind={:?}", self.kind);
        put_defs_refs_here.sort_by(|a, b| if a.1 < b.1
        // row differs
        {
            Ordering::Less
        } else if a.1 == b.1 {
            // same row
            if a.0 <= b.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            {
                Ordering::Greater
            }
        });

        if self.kind == "html" {
            let mut ref_def_row: usize = 0;
            for a_row_nr in 0..self.all_rows.len() {
                //        println!("",);
                if ref_def_row >=  (put_defs_refs_here.len() - 0) // no more corrections
                    || a_row_nr < put_defs_refs_here[ref_def_row].1
                // .1 is row
                {
                    // no replace here
                    let s: String = self.all_rows[a_row_nr].iter().cloned().collect();
                    putdatarowshere.push(s.clone());
                //                    println!("{}", s)
                } else {
                    // split source
                    // original 1111111<>2222222<>33333333<>444
                    // merge this in AAAA<7>,BBBB<15>,CCC<23>
                    //
                    // giving 1111111<AAAA>2222222<BBBB>33333333<CCC>444
                    let mut mod_row: Vec<String> = Vec::new();
                    let mut last_off = put_defs_refs_here[ref_def_row].0;
                    let first;
                    let mut rest;
                    let first_and_rest =
                        self.all_rows[a_row_nr].split_at(put_defs_refs_here[ref_def_row].0); // .0 is col
                    first = first_and_rest.0;
                    rest = first_and_rest.1;
                    mod_row.push(first.iter().cloned().collect::<String>());

                    let mut html_txt: String;
                    html_txt = BNF::insert_anchor(
                        put_defs_refs_here[ref_def_row].2,
                        put_defs_refs_here[ref_def_row].3.clone(),
                    );
                    mod_row.push(html_txt.clone()); //// AAAAAA
                    ref_def_row += 1;
                    while ref_def_row <= (put_defs_refs_here.len() - 1) &&
                        a_row_nr == put_defs_refs_here[ref_def_row].1
                    // same row
                    {
                        let split_pos = put_defs_refs_here[ref_def_row].0 - last_off;
                        last_off = put_defs_refs_here[ref_def_row].0;
                        let first_and_rest2 = rest.split_at(split_pos);
                        mod_row.push(first_and_rest2.0.iter().cloned().collect::<String>()); // 2222222 or 33333333
                        html_txt = BNF::insert_anchor(
                            put_defs_refs_here[ref_def_row].2,
                            put_defs_refs_here[ref_def_row].3.clone(),
                        );
                        mod_row.push(html_txt.clone());
                        ref_def_row += 1;
                        rest = first_and_rest2.1;
                    }
                    mod_row.push(rest.iter().cloned().collect::<String>()); // 4444
                    putdatarowshere.push(mod_row.join("").clone());
                }
            }
        } else {
            for i in 0..self.all_rows.len() {
                let s: String = self.all_rows[i].iter().cloned().collect();
                putdatarowshere.push(s);
            }
        }
    }
    /*................................................*/
    fn brute_stop(&mut self, expected: String) {
        let from: usize;
        let to: usize;
        if self.at as isize - 10 < 0 {
            from = 0
        } else {
            from = self.at - 10
        };
        if self.at + 10 > self.the_grammer_as_rules.len() {
            to = self.the_grammer_as_rules.len()
        } else {
            to = self.at + 10
        };
        panic!(
            "Found '{}' but expecting '{}' close to {:#?}",
            self.ch as char,
            expected,
            String::from_utf8_lossy(&self.the_grammer_as_rules[from..to])
        );

    }
    /*................................................*/
    fn next(&mut self, look_for: u8) {
        if look_for != 128 && look_for != self.ch {
            self.brute_stop(String::from_utf8(vec![look_for]).unwrap());
            //            panic!(
            //                "Expecting \"{}\" but found \"{}\"",
            //                look_for as char,
            //                self.ch as char
            //            )
        }
        if self.at >= self.the_grammer_as_rules.len() {
            self.ch = 0;
        } else {
            self.ch = self.the_grammer_as_rules[self.at];
            self.at += 1;
        }
    }
    /*................................................*/
    fn white(&mut self) {
        while self.ch <= ' ' as u8 && self.ch != 0 {
            self.next(128);
        }
    }
    /*................................................*/
    fn single_quoted(&mut self) -> String {
        let mut val = String::new();
        self.next(128);
        while self.ch != '\'' as u8 && self.ch != 0 {
            val.push(self.ch as char);;
            self.next(128);
        }
        self.next('\'' as u8);
        self.white();
        return val;
    }
    /*................................................*/
    fn double_qouted(&mut self) -> (bool, String) {
        let mut val = String::new();
        self.next(128);
        while self.ch != '\"' as u8 && self.ch != 0 {
            val.push(self.ch as char);;
            self.next(128);
        }
        self.next('"' as u8);
        self.white();
        let is_reserved = self.the_word_list.borrow().is_res_word(val.clone());
        if is_reserved {
            return (true, val);
        } else {
            return (false, val);
        }
    }
    /*................................................*/
    fn identifier(&mut self, a_ref: bool) -> String {
        let mut id: String = String::new();
        if (self.ch >= 'a' as u8 && self.ch <= 'z' as u8) ||
            (self.ch >= 'A' as u8 && self.ch <= 'Z' as u8) || self.ch == '_' as u8
        {
            while (self.ch >= 'a' as u8 && self.ch <= 'z' as u8) ||
                (self.ch >= 'A' as u8 && self.ch <= 'Z' as u8) ||
                self.ch == '_' as u8 ||
                (self.ch >= '0' as u8 && self.ch <= '9' as u8)
            {
                id.push(self.ch as char);
                self.next(128);
            }
        }
        self.white();
        if id.len() == 0 {
            self.brute_stop("identifier".to_string());

            //          let from:usize;
            //          let to:usize;
            //            				if self.at as isize- 10 < 0  {from =0} else {from =self.at - 10};
            //            				if self.at + 10 > self.the_grammer_as_rules.len() {to=self.the_grammer_as_rules.len()} else { to=self.at + 10};
            //            panic!("Found '{}' but expecting an identifier close to {:#?}", self.ch as char,
            //             String::from_utf8_lossy(&self.the_grammer_as_rules[from..to]));
        }
        if a_ref {
            let mut _the_def_ref_dict = self.def_ref_check_results.borrow_mut();
            refsdefs::RefsDefs::insert_ref(
                &mut _the_def_ref_dict,
                self.grammer_file_name.clone(),
                id.to_string(),
            );
        }
        return id;
    }
    /*................................................*/
    fn number(&mut self) -> String {
        let mut id = String::new();
        while self.ch >= '0' as u8 && self.ch <= '9' as u8 {
            id.push(self.ch as char);
            self.next(128);
        }
        self.white();
        return id;
    }
    /*................................................*/
    fn grammer(&mut self) {
        self.white();
        while self.ch != 0 {
            self.white();
            let a_rule = self.rule();
            self.all_rule_objs.push(Rc::new(RefCell::new(a_rule)));
        }
    }
    /*................................................*/
    fn rule(&mut self) -> Rule {
        let mut the_rule = Bocks::new("rule".to_string(), "".to_string());
        let nonterminal = self.identifier(false);
        the_rule.text_content = nonterminal.clone();
        self.next(':' as u8);
        self.white();
        let prod_rule = self.productionrule();
        self.next(';' as u8);
        self.white();
        let a_rule_obj = Rule::new(the_rule, prod_rule);
        let mut _the_def_ref_dict = self.def_ref_check_results.borrow_mut();
        refsdefs::RefsDefs::insert_def(
            &mut _the_def_ref_dict,
            self.grammer_file_name.clone(),
            nonterminal.clone().to_string(),
        );
        return a_rule_obj;
    }
    /*................................................*/
    fn productionrule(&mut self) -> Bocks {
        // a b c | d
        // (a b) is seuence and choice is (c,d)
        let sequence = self.production();
        self.white();
        if self.ch == '|' as u8 {
            // pick last fromo sequence as first in choice
            let mut the_choice = Bocks::new("choice".to_string(), "".to_string());
            the_choice.kids.push(Rc::new(RefCell::new(sequence)));
            while self.ch == '|' as u8 {
                self.next(128);
                let p = self.production();
                the_choice.kids.push(Rc::new(RefCell::new(p)));
                self.white();
            }
            return the_choice;
        } else {
            return sequence;
        };
    }
    /*................................................*/
    fn production(&mut self) -> Bocks {
        let t = self.term(); // zero or more
        return t;
    }
    /*................................................*/
    fn term(&mut self) -> Bocks {
        let mut seq = Bocks::new("sequence".to_string(), "".to_string());
        while !(self.ch == '|' as u8 || self.ch == ';' as u8 || self.ch == ']' as u8 ||
                    self.ch == ')' as u8 || self.ch == 0)
        {
            let mut elm = self.element();
            elm = self.repeats(elm);
            seq.kids.push(Rc::new(RefCell::new(elm)));
        }
        return seq;
    }
    /*................................................*/
    fn element(&mut self) -> Bocks {
        let mut elm: Bocks;
        self.white();
        if self.ch == '\'' as u8 {
            let val = self.single_quoted();
            elm = Bocks::new("single_quoted".to_string(), val);
        } else if self.ch == '"' as u8 {
            let val = self.double_qouted();
            if val.0 == false {
                elm = Bocks::new("double_qouted".to_string(), val.1);
            } else {
                elm = Bocks::new("reserved_word".to_string(), val.1);
            }
        } else if self.ch == '[' as u8 {
            elm = Bocks::new("group".to_string(), "".to_string());
            self.next(128);
            self.white();
            let prod_rule = self.productionrule();
            elm.kids.push(Rc::new(RefCell::new(prod_rule)));
            self.next(']' as u8);
            self.white();
            elm = self.repeats(elm);
        } else if self.ch == '(' as u8 {
            elm = Bocks::new("group".to_string(), "".to_string());
            self.next(128);
            self.white();
            let prod_rule = self.productionrule();
            elm.kids.push(Rc::new(RefCell::new(prod_rule)));
            self.next(')' as u8);
            self.white();
            elm = self.repeats(elm);
        } else {
            let ident = self.identifier(true);
            elm = Bocks::new("identifier".to_string(), ident);
        }
        return elm;
    }
    /*................................................*/
    fn repeats(&mut self, prev_box: Bocks) -> Bocks {
        let mut iter_box: Bocks;
        if self.ch == '*' as u8 || self.ch == '+' as u8 {
            if self.ch == '+' as u8 {
                iter_box = Bocks::new("iter_one_or_more".to_string(), "".to_string());
            } else {
                iter_box = Bocks::new("iter_zero_or_more".to_string(), "".to_string());
            }
            self.next(128);
            self.white();
            if self.ch >= '1' as u8 && self.ch <= '9' as u8 {
                let nr = self.number();
                iter_box.occurs = nr.clone();
                iter_box.max_iter = nr;
            } else {
                iter_box.occurs = "1".to_string();
                iter_box.max_iter = "1".to_string();
            }
            iter_box.kids.push(Rc::new(RefCell::new(prev_box)));
            return iter_box;
        } else if self.ch >= '1' as u8 && self.ch <= '9' as u8 {
            // on its own
            // just a number
            iter_box = Bocks::new("iter_one_or_more".to_string(), "".to_string());
            let nr = self.number();
            iter_box.occurs = nr.clone();
            iter_box.max_iter = nr;
            iter_box.kids.push(Rc::new(RefCell::new(prev_box)));
            return iter_box;
        } else if self.ch == '?' as u8 {
            iter_box = Bocks::new("optional".to_string(), "".to_string());
            iter_box.occurs = "1".to_string();
            iter_box.max_iter = "1".to_string();
            self.next(128);
            self.white();
            iter_box.kids.push(Rc::new(RefCell::new(prev_box)));
            return iter_box;
        } else if self.ch == ',' as u8 {
            self.next(128);
            self.white();
            if self.ch == '\'' as u8 {
                let val = self.single_quoted();
                iter_box = Bocks::new("iter_with_list_separator".to_string(), val);
                iter_box.kids.push(Rc::new(RefCell::new(prev_box)));
                return iter_box;
            } else {
                self.brute_stop("literal".to_string());
                return prev_box;
            }
        } else {
            return prev_box;
        }
    }
    fn insert_anchor(kind: &'static str, some_text: String) -> String {
        // def is just <a name=''>
        // we have ---TEXT---
        // we want ---<a href='#TEXT'>TEXT</a>---
        // ref1       ZZZZZZZZZZZZZ
        // ref2                           ZZZZ

        match kind {
            "def" => format!("<a name='{}'></a>", some_text),
            "ref1" => format!("<a href='#{}'>", some_text),
            "ref2" => format!("{}", some_text),
            _ => "".to_string(),
        }
    }
}
