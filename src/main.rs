//mod bnf;
//use bnf::BNF;
mod ebnf;
use ebnf::EBNF;
mod rule;
mod bocks;
mod reswords;
use reswords::Reswords;
use std::env;
use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;
//mod matchpattern;

#[derive(Debug)]
pub struct StOptional {
    //-[>----->]-
    // [>- s ->]
    first_row: Vec<&'static str>,
    second_row: Vec<&'static str>,
}
pub struct StChoice {
    //-[>--c1-->]-
    // [        ]
    // [        ]
    // [>- c2 ->]
    first_row: Vec<&'static str>,
    other_rows: Vec<&'static str>,
    empty_rows: Vec<&'static str>,
}
pub struct StIterZeroOrMore {
    //-[>----------->]-
    // [>-{>- s ->}->]
    //    {       }
    //    {<-----<}
    first_row: Vec<&'static str>,
    second_row: Vec<&'static str>,
    empty_rows: Vec<&'static str>,
    back_row: Vec<&'static str>,
}
pub struct StIterOneOrMore {
    //-{>- ssssssssss ->}-
    // {                }
    // {<-- separator -<}
    first_row: Vec<&'static str>,
    empty_rows: Vec<&'static str>,
    back_row: Vec<&'static str>,
}
pub struct TTYStyle {
    the_opt: StOptional,
    the_choice: StChoice,
    the_iter_zero_or_more: StIterZeroOrMore,
    the_iter_one_or_more: StIterOneOrMore,
}
pub struct SVGStyle {
    scale_x: f32,
    scale_y: f32,
    line_dist: f32,
    radie_lines: f32,
    rect_corner_radie: f32,
}
pub struct Dims {
    tty_style: TTYStyle,
    svg_style: SVGStyle,
}
mod grammarmother;
use grammarmother::Grammarmother;
//mod grammarrefdefs;
mod refsdefs;
//use refsdefs::RefsDefs;
#[derive(Debug)]
pub enum Kind {
    Version,
    Novalue,
    Grammar,
    Groupgrammar,
    InfoOnly,
    Help,
}
#[derive(Debug)]
struct Input {
    kind: Kind,
    file_name_keywords: String,
    in_grammar_filename: String,
    in_group_grammar_filename: String,
    in_cssstyle_filename: String,
    in_dest_print: String,
    out_kind: String,
    in_gen_rulename: String,
    in_gen_seed: String,
    in_max_steps: String,
}
type ArgsResult = Result<Input, String>;

const USAGE: &'static str = "
 command  : -[>- VER --->]-                                                               
             [>- HELP -->]                                                                
             [>- CHECK ->]                                                                
             [>- VIEW -->]                                                                
             [>- GEN --->]                                                                
                                                                                          
 VER      : - \"--version\" -                                                               
                                                                                          
 HELP     : - \"--help\" -                                                                  
                                                                                          
 CHECK    : - \"--check\" -- GRAMMER -                                                      
                                                                                          
 VIEW     : -[>- \"--tty\" -->]-- GRAMMER --[>------------>]--[>------->]-                  
             [>- \"--svg\" -->]             [>- RESWORDS ->]  [>- OUT ->]                   
             [>- \"--html\" ->]                                                             
                                                                                          
 GRAMMER  : -[>-[>- \"--grammar\" ->]-- grammar_file_name ------------------------>]-       
             [  [>- \"--gr\" ------>]                                              ]        
             [>-[>- \"--groupgrammar\" ->]-- file_with_set_of_grammar_file_names ->]        
                [>- \"--grgr\" --------->]                                                  
                                                                                          
 RESWORDS : - \"--reswords\" -- file_with_reserved_symbols -                                
                                                                                          
 OUT      : - \"--out\" -- out_file_name -                                                  
                                                                                          
 SEED     : - \"--seed\" -- seednumber -                                                    
                                                                                          
 MAXSTEPS : - \"--maxsteps\" -- number -                                                    
                                                                                          
 GEN      : - \"--genrule\" -- rule_name --[>------------>]--[>-------->]--[>------------>]-
                                         [>- RESWORDS ->]  [>- SEED ->]  [>- MAXSTEPS ->]
            ";
/*---------------- pick_args ------------------------------*/
fn pick_args() -> ArgsResult {
    let mut my_input = Input {
        kind: Kind::Novalue,
        file_name_keywords: "".to_string(),
        in_grammar_filename: "".to_string(),
        in_group_grammar_filename: "".to_string(),
        in_dest_print: "".to_string(),
        out_kind: "tty".to_string(), // default
        in_cssstyle_filename: "".to_string(),
        in_gen_rulename: "".to_string(),
        in_gen_seed: "0".to_string(),
        in_max_steps: "20".to_string(),
    };
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{}", USAGE);
        return Result::Err("Missing arguments".to_string());
    };
    let mut pos = 1;
    let args_len = args.len();
    while pos < args_len {
        match args[pos].as_str() {
            "--version" => {
                my_input.kind = Kind::Version;
                return Result::Ok(my_input);
            }
            "--help" => {
                my_input.kind = Kind::Help;
                return Result::Ok(my_input);
            }
            "--tty" => my_input.out_kind = "tty".to_string(),
            "--svg" => my_input.out_kind = "svg".to_string(),
            "--html" => my_input.out_kind = "html".to_string(),
            "--check" => my_input.out_kind = "check".to_string(),
            "--genrule" => {
                my_input.out_kind = "genrule".to_string();
                pos += 1;
                if pos < args_len {
                    my_input.in_gen_rulename = args[pos].clone();
                } else {
                    return Result::Err("Missing genrulename".to_string());
                }
            }
            "--seed" => {
                pos += 1;
                if pos < args_len {
                    my_input.in_gen_seed = args[pos].clone();
                } else {
                    return Result::Err("Missing seed value".to_string());
                }
            }
            "--maxsteps" => {
                pos += 1;
                if pos < args_len {
                    my_input.in_max_steps = args[pos].clone();
                } else {
                    return Result::Err("Missing maxsteps value".to_string());
                }
            }
            "--grammar" | "--gr" => {
                pos += 1;
                if pos < args_len {
                    my_input.in_grammar_filename = args[pos].clone();
                    my_input.kind = Kind::Grammar
                } else {
                    return Result::Err("Missing grammarfile".to_string());
                }
            }
            "--groupgrammar" | "--grgr" => {
                pos += 1;
                if pos < args_len {
                    my_input.in_group_grammar_filename = args[pos].clone();
                    my_input.kind = Kind::Groupgrammar
                } else {
                    return Result::Err("Missing groupgrammarfilename".to_string());
                }
            }
            "--out" => {
                pos += 1;
                if pos < args_len {
                    my_input.in_dest_print = args[pos].clone();
                } else {
                    return Result::Err("Missing outputfilename".to_string());
                }
            }
            "--cssstyle" => {
                pos += 1;
                if pos < args_len {
                    my_input.in_cssstyle_filename = args[pos].clone();
                } else {
                    return Result::Err("Missing cssstylefilename".to_string());
                }
            }
            "--reswords" => {
                pos += 1;
                if pos < args_len {
                    my_input.file_name_keywords = args[pos].clone();
                } else {
                    return Result::Err("Missing reswords filename".to_string());
                }
            }
            _ => {
                let msg: String = format!(
                    "Can't understand '{}'\n{}",
                    args[pos].as_str(),
                    "Try --help"
                );
                return Result::Err(msg.to_string());
            }
        }
        pos += 1
    }
    Result::Ok(my_input)
}
/*-------------- read_file_content --------------------------------*/
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use std::io::{self, Write};

pub fn read_file_content(filename: &String) -> Result<String, String> {
    match File::open(filename) {
        Err(ref why_open) => {
            Err(
                format!(
                    "Couldn't open file '{}': {}",
                    filename,
                    why_open.description()
                ).to_string(),
            )
        }
        Ok(mut the_file) => {
            let mut contents = String::new();
            match the_file.read_to_string(&mut contents) {
                Err(why_read) => {
                    Err(format!(
                        "Couldn't read the file '{}': {}",
                        filename,
                        why_read.description()
                    ))
                }
                Ok(_ok_read) => Ok(contents),
            }
        }
    }
}
/*................ create_out_writer ..............*/
fn create_out_writer(file_name_out: String) -> Box<Write> {
    if file_name_out.len() == 0 {
        Box::new(io::stdout()) as Box<Write>
    } else {
        let path = Path::new(&file_name_out);
        Box::new(File::create(&path).unwrap()) as Box<Write>
    }
}
/*................ parse_grammers ..............*/
fn parse_grammers(
    config_start_x: usize,
    config_start_y: usize,
    the_mother: &Grammarmother,
) -> (usize, usize) {
    let mut calc_start_y: usize = config_start_y;
    let mut _vidd: usize = 0;
    let mut vidder: Vec<usize> = Vec::new();
    for i in 0..the_mother.ebnf_kids.len() {
        let mut a_ebnfobj = the_mother.ebnf_kids[i].borrow_mut();
        let mut vidd_titel: f32 = format!("Grammar filename: {}", a_ebnfobj.grammer_file_name)
            .len() as f32;
        if the_mother.kind == "svg".to_string() {
            vidd_titel *= 1.7;
        }
        vidder.push(vidd_titel as usize);
        a_ebnfobj.parse_grammer();
        let sizes = a_ebnfobj.calc(config_start_x, calc_start_y, vidd_titel as usize);
        _vidd = sizes.0; // find max for all rules in this grammar
        vidder.push(_vidd);
        calc_start_y += sizes.1; // height of this grammer
        calc_start_y += 2; // grammer spacing
        if the_mother.kind == "tty".to_string() || the_mother.kind == "html".to_string() {
            calc_start_y = config_start_y; // restart don't accumulate
        }
    }
    let max_vidd: usize = vidder.iter().map(|x| *x).max().unwrap();
    (calc_start_y, max_vidd)
}
/*................ prod_out_put ..............*/
fn prod_out_put(
    the_mother: &mut Grammarmother,
    config_start_x: usize,
    config_start_y: usize,
    max_vidd: usize,
    total_hojd: usize,
    out_writer: &mut Box<Write>,
) {
    let mut putdatarowshere: Vec<String> = Vec::new();
    if the_mother.kind == "svg".to_string() {
        the_mother.gen_svg_pre(max_vidd, total_hojd, &mut putdatarowshere);
    }
    let mut start_y: usize = config_start_y;
    for i in 0..the_mother.ebnf_kids.len() {
        let mut a_ebnfobj = the_mother.ebnf_kids[i].borrow_mut();
        match the_mother.kind.as_str() {
            "tty" | "html" => {
                a_ebnfobj.as_tty(
                    config_start_x,
                    start_y,
                    &the_mother.dims.tty_style,
                    &mut putdatarowshere,
                );
            }
            "svg" => {
                start_y = a_ebnfobj.as_svg(
                    config_start_x,
                    start_y,
                    &the_mother.dims,
                    &mut putdatarowshere,
                );
                start_y += 2; // grammer spacing
            }
            "genrule" => {}
            _ => panic!("the_mother.kind unknown {:?}", the_mother.kind),
        }
    }
    match the_mother.kind.as_str() {
        "tty" => {
            for a_row in putdatarowshere {
                write!(out_writer, "{}\n", a_row).unwrap();
            }
        }
        "html" => {
            write!(out_writer, "{}\n", "<pre>").unwrap();
            for a_row in putdatarowshere {
                write!(out_writer, "{}\n", a_row).unwrap();
            }
        }
        "svg" => {
            the_mother.gen_svg_post(&mut putdatarowshere);
            for a_row in putdatarowshere {
                write!(out_writer, "{}", a_row).unwrap();
            }
        }
        "genrule" => {}
        _ => panic!("the_mother.kind unknown {:?}", the_mother.kind),
    }
}
/*................ main ..............*/
fn main() {
    // 1. Set up res words
    // 2. Prepare a collector for defs and refs in  rules
    // 3. Pick arguments from command line
    // 4. Create a destinaion for output
    // 5. Parse grammars and calculate widths and heigths
    // 6. Produce output

    let collect_defs_and_refs = Rc::new(RefCell::new(refsdefs::RefsDefs::new()));

    let mut the_mother = Grammarmother::new();
    let from_args = pick_args();

    let mut file_name_out: String = String::new();
    let mut use_gen_rulename: String = String::new();
    let mut use_gen_seed: String = String::new();
    let mut use_max_steps: String = String::new();

    let args_result: Result<String, String> = match from_args {
        Ok(an_input) => {
            let res_words = Rc::new(RefCell::new(Reswords::new()));
            let resword_result = res_words.borrow_mut().set_up_once(
                an_input.file_name_keywords,
            );
            // did resword failed???
            let resword_result: Result<String, String> = match resword_result {
                Ok(oktext) => Result::Ok(oktext),
                Err(errtext) => Result::Err(errtext),
            };
            ///////////////
            if resword_result.is_ok() {
                file_name_out = an_input.in_dest_print;
                use_gen_rulename = an_input.in_gen_rulename;
                use_gen_seed = an_input.in_gen_seed;
                use_max_steps = an_input.in_max_steps;

                let input_kind_result: Result<String, String> = match an_input.kind {
                    Kind::Version => Result::Ok("version 1".to_string()),
                    Kind::Novalue => Result::Err(
                        "Missing input --grammer or --groupgrammer".to_string(),
                    ),
                    Kind::Help | Kind::InfoOnly => Result::Ok(USAGE.to_string()),
                    Kind::Grammar => {
                        the_mother.set_kind(an_input.out_kind.clone());
                        // open one file and process
                        let one_grammar_read_result: Result<String, String>;
                        one_grammar_read_result =
                            match read_file_content(&an_input.in_grammar_filename) {
                                Err(why) => Result::Err(why),
                                Ok(content) => {
                                    //                                    let a_ebnfobj = BNF::new(
                                    let a_ebnfobj = EBNF::new(
                                        an_input.in_grammar_filename,
                                        content,
                                        res_words.clone(),
                                        collect_defs_and_refs.clone(),
                                        an_input.out_kind.clone(),
                                    );
                                    the_mother.ebnf_kids.push(Rc::new(RefCell::new(a_ebnfobj)));
                                    Result::Ok("".to_string())
                                }
                            };
                        one_grammar_read_result
                    }
                    Kind::Groupgrammar => {
                        // open one file and process each row as a grammer
                        the_mother.set_kind(an_input.out_kind.clone());
                        let path = Path::new(&an_input.in_group_grammar_filename);
                        let display = path.display();
                        // https://doc.rust-lang.org/std/path/
                        let parent = path.parent()
                            .unwrap()
                            .as_os_str()
                            .to_str()
                            .unwrap()
                            .to_string();
                        let one_grgr_open_result: Result<String, String>;
                        one_grgr_open_result = match File::open(&path) {
                            Err(why) => Result::Err(
                                format!("couldn't open {}: {}", display, why.description()),
                            ),
                            Ok(f2) => {
                                let f3 = BufReader::new(f2);
                                let mut all_grgr_file_read_result: Result<
                                    String,
                                    String,
                                > = Result::Ok("Not Yet".to_string());
                                for line in f3.lines() {
                                    let the_grammar_file_name = line.unwrap();
                                    let dir_sep;
                                    if parent.len() == 0 {
                                        dir_sep = "";
                                    } else {
                                        dir_sep = "/";
                                    }
                                    let full_file_name =
                                        format!("{}{}{}", parent, dir_sep, the_grammar_file_name);
                                    all_grgr_file_read_result =
                                        match read_file_content(&full_file_name) {
                                            Err(why) => Result::Err(why),
                                            Ok(content) => {
                                                let a_ebnfobj = EBNF::new(
                                                    the_grammar_file_name.clone(),
                                                    content,
                                                    res_words.clone(),
                                                    collect_defs_and_refs.clone(),
                                                    an_input.out_kind.clone(),
                                                );
                                                the_mother.ebnf_kids.push(Rc::new(
                                                    RefCell::new(a_ebnfobj),
                                                ));
                                                Result::Ok("".to_string())
                                            }
                                        };
                                    // if failure here terminate the loop
                                    if all_grgr_file_read_result.is_err() {
                                        break;
                                    }
                                }
                                all_grgr_file_read_result
                            }
                        };
                        one_grgr_open_result
                    }
                };
                input_kind_result
            } else {
                resword_result
            }
        }
        Err(why) => Result::Err(why),
    };

    match args_result {
        Ok(res) => {
            if res.len() != 0 {
                println!("{}", res);
            } else {
                let mut out_writer = create_out_writer(file_name_out.clone());
                let config_lines_above: usize = 1;
                let config_start_x: usize = 1;
                let config_start_y: usize = config_lines_above + 3; // filename
                let (total_hojd, max_vidd) =
                    parse_grammers(config_start_x, config_start_y, &the_mother);
                match the_mother.kind.as_str() {
                    "check" => {
                        refsdefs::RefsDefs::check(&mut collect_defs_and_refs.borrow_mut());
                    }
                    "genrule" => {
                        if the_mother.ebnf_kids.len() != 1 {
                            panic!("genrule only works on one grammar file, nor a set of files");
                        } else {
                            // find initial rule
                            // send an empty vector for strings
                            // print result
                            let mut putgentexthere: String = String::new();
                            let seed: usize = use_gen_seed.parse().unwrap();
                            let max_steps: usize = use_max_steps.parse().unwrap();
                            the_mother.ebnf_kids[0].borrow_mut().find_rule_by_name(
                                use_gen_rulename.as_str(),
                                &mut putgentexthere,
                                seed, // seed start
                                max_steps,
                            );
                            write!(out_writer, "{}\n", putgentexthere).unwrap();
                            //                            for a_row in putgentexthere {
                            //                                write!(out_writer, "{}\n", a_row).unwrap();
                            //                            }
                        }
                    }
                    _ => {
                        prod_out_put(
                            &mut the_mother,
                            config_start_x,
                            config_start_y,
                            max_vidd,
                            total_hojd,
                            &mut out_writer,
                        );
                    }
                }
            }
        }
        Err(err) => println!("Well, {}", err),
    }
}
