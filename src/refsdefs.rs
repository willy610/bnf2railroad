use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::btree_map::Entry::{Occupied, Vacant};

/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
#[derive(Debug)]
pub struct RefsDefs {
    // key is def and data is file name.
    // check duplicates of def in several files!!
    pub the_defs_in_file: BTreeMap<String, BTreeSet<String>>,
    // key is file:ref
    //  pub the_refs_in_file: HashSet<String>, // don't use this. use xxx
    pub the_refs_in_file: BTreeMap<String, BTreeSet<String>>,
}
impl RefsDefs {
    pub fn new() -> RefsDefs {
        RefsDefs {
            the_defs_in_file: BTreeMap::new(),
            the_refs_in_file: BTreeMap::new(),
        }
    }
    /*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
    pub fn insert_ref(&mut self, file: String, ref_name: String) {
        let mut hash_set_old_files = match self.the_refs_in_file.entry(ref_name.clone()) {
            Vacant(vac_entry) => {
                let mut file_names = BTreeSet::new();
                file_names.insert(file.clone());
                // insert and return the old hashset
                // VacantEntry returns
                vac_entry.insert(file_names)
            }
            Occupied(occ_entry) => {
                // return the existing hashset
                // OccupiedEntry returns
                occ_entry.into_mut()
            }
        };
        // this def might occcur in several file by mistake!
        hash_set_old_files.insert(file.clone());
        ////////////////////////
    }
    /*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
    pub fn insert_def(&mut self, file: String, def_name: String) {
        // try insert the def
        let mut hash_set_old_files = match self.the_defs_in_file.entry(def_name.clone()) {
            Vacant(vac_entry) => {
                let mut file_names = BTreeSet::new();
                file_names.insert(file.clone());
                // insert and return the old hashset
                // VacantEntry returns
                vac_entry.insert(file_names)
            }
            Occupied(occ_entry) => {
                // return the existing hashset
                // OccupiedEntry returns
                occ_entry.into_mut()
            }
        };
        // this def might occcur in several file by mistake!
        hash_set_old_files.insert(file.clone());
    }
    /*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
    pub fn check(&mut self) {
        /*
ref,fromfile as ref_rel
def,infile   as def_rel

find all undefined
-----------------
select distinct * from refrel
 where refrel.ref not in 
 (select def from defrel)

find all unused
--------------
select distinct * from defrel
 where defrel.def not in 
 (select ref from refrel)

find duplicate definitions
-----------------
select * from 
(select distinct * from
 defrel join refrel on refrel.ref = defrel.def) as dist
group by dist.def,dist.ref
 having count(*) > 1
    */
        // ensure all refs are defined
        let mut out_undefined: Vec<(String, Vec<String>)> = Vec::new();
        let mut out_ambig: Vec<(String, Vec<String>, Vec<String>)> = Vec::new();
        let mut out_unused: Vec<(String, Vec<String>)> = Vec::new();
        let dump_sql = false;
        if dump_sql {
            let mut sql_values: Vec<String> = Vec::new();
            for (ref_name, ref_in_file) in &self.the_refs_in_file {
                for a_ref_file_name in ref_in_file {
                    sql_values.push(format!("('{}','{}')", ref_name, a_ref_file_name))
                }
            }
            println!(
                "TRUNCATE TABLE refrel;
INSERT INTO refrel(`ref`,`fromfile`) VALUES {};",
                sql_values.join(",\n")
            );
            let mut sql_values: Vec<String> = Vec::new();
            for (ref_name, from_in_file) in &self.the_defs_in_file {
                for a_from_file_name in from_in_file {
                    sql_values.push(format!("('{}','{}')", ref_name, a_from_file_name))
                }
            }
            println!(
                "TRUNCATE TABLE defrel;
INSERT INTO defrel(`def`,`infile`) VALUES {};",
                sql_values.join(",\n")
            );
        }
        /////////////////////////////
        //        println!("the_refs_in_file(0)={:?}", self.the_refs_in_file);
        for (a_ref_symb, a_ref_symb_in_file_names) in &mut self.the_refs_in_file {
            let a_ref_symb_in_file_names_as_vec = a_ref_symb_in_file_names
                .iter()
                .cloned()
                .collect::<Vec<String>>();
            // look into definitions for this 'a_ref_symb'
            let the_possible_def_of_a_ref = self.the_defs_in_file.entry(a_ref_symb.clone());
            match the_possible_def_of_a_ref {
                Vacant(_vac_entry) => {
                    // not defined
                    out_undefined.push((a_ref_symb.clone(), a_ref_symb_in_file_names_as_vec));
                }
                Occupied(occ_entry) => {
                    // where is the symbol defined
                    let the_def_files_of_a_ref = occ_entry.into_mut();
                    if the_def_files_of_a_ref.len() > 1 {
                        // having count(*) > 1 -> in several files
                        let the_def_files_of_a_ref_as_vec = the_def_files_of_a_ref
                            .iter()
                            .cloned()
                            .collect::<Vec<String>>();
                        out_ambig.push((
                            a_ref_symb.clone(),
                            a_ref_symb_in_file_names_as_vec,
                            the_def_files_of_a_ref_as_vec,
                        ))
                    }
                }
            }
        }
        /////////////////////////////
        for (def_name, def_in_file) in &mut self.the_defs_in_file {
            let the_possible_ref = self.the_refs_in_file.entry(def_name.clone());
            let def_file_names = def_in_file.iter().cloned().collect::<Vec<String>>();
            match the_possible_ref {
                Vacant(_vac_entry) => {
                    // not used
                    out_unused.push((def_name.clone(), def_file_names));
                }
                Occupied(_occ_entry) => {
                    // in use. nop
                }
            }
        }
        //    println!("out_undefined={:?}", out_undefined);
        //    println!("out_ambig={:?}", out_ambig);
        //    println!("out_unused={:?}", out_unused);

        self.show_undef(&mut out_undefined);
        self.show_unused(&mut out_unused);
        self.show_ambig(&mut out_ambig);
    }
    /*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
    fn show_undef(&self, out_undefined: &mut Vec<(String, Vec<String>)>) {
        let mut max_vidd_ref_undef: usize = "Symbol".len();
        let mut max_vidd_file_undefs: usize = "Referenced in file(s)".len();
        // calc column widths
        for an_undef_indx in 0..out_undefined.len() {
            let an_undef = &out_undefined[an_undef_indx];
            if an_undef.0.len() > max_vidd_ref_undef {
                max_vidd_ref_undef = an_undef.0.len();
            }
            let max_vidd_these_file_undefs = 0 +
                an_undef.1.iter().map(|x| x.len()).fold(
                    0,
                    |max_sofar, i| if i >
                        max_sofar
                    {
                        i
                    } else {
                        max_sofar
                    },
                );
            if max_vidd_these_file_undefs > max_vidd_file_undefs {
                max_vidd_file_undefs = max_vidd_these_file_undefs;
            }
        }
        // and print
        println!(
            "\nUndefined definitions:
======================"
        );
        println!(
            " {:vidd_1$} | {:vidd_2$} |",
            "Symbol",
            "Referenced in file(s)",
            vidd_1 = max_vidd_ref_undef,
            vidd_2 = max_vidd_file_undefs
        );
        println!(
            " +{:->vidd$}+",
            "-",
            vidd = max_vidd_ref_undef + 3 + max_vidd_file_undefs
        );
        for an_undef_indx in 0..out_undefined.len() {
            let (ref an_undef, ref file_list) = out_undefined[an_undef_indx];
            // first row
            println!(
                " {:vidd_1$} | {:vidd_2$} |",
                an_undef,
                file_list[0],
                vidd_1 = max_vidd_ref_undef,
                vidd_2 = max_vidd_file_undefs
            );
            // rest of rows
            for rest_rows_index in 1..file_list.len() {
                println!(
                    " {:vidd_1$} | {:vidd_2$} |",
                    "",
                    file_list[rest_rows_index],
                    vidd_1 = max_vidd_ref_undef,
                    vidd_2 = max_vidd_file_undefs
                );
            }
        }
    }
    /*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
    fn show_unused(&self, out_unused: &mut Vec<(String, Vec<String>)>) {
        let mut max_vidd_ref_unused: usize = "Symbol".len();
        let mut max_vidd_file_unused: usize = "Defined in file(s)".len();
        // calc column widths
        for an_unused_indx in 0..out_unused.len() {
            let an_unused = &out_unused[an_unused_indx];
            if an_unused.0.len() > max_vidd_ref_unused {
                max_vidd_ref_unused = an_unused.0.len();
            }
            let max_vidd_these_file_unuseds = 0 +
                an_unused.1.iter().map(|x| x.len()).fold(
                    0,
                    |max_sofar, i| if i >
                        max_sofar
                    {
                        i
                    } else {
                        max_sofar
                    },
                );
            if max_vidd_these_file_unuseds > max_vidd_file_unused {
                max_vidd_file_unused = max_vidd_these_file_unuseds;
            }
        }
        // and print
        println!(
            "\nUnused definitions:
======================"
        );
        println!(
            " {:vidd_1$} | {:vidd_2$} |",
            "Symbol",
            "Defined in file(s)",
            vidd_1 = max_vidd_ref_unused,
            vidd_2 = max_vidd_file_unused
        );
        println!(
            " +{:->vidd$}+",
            "-",
            vidd = max_vidd_ref_unused + 3 + max_vidd_file_unused
        );
        for an_unused_indx in 0..out_unused.len() {
            let (ref an_unused, ref file_list) = out_unused[an_unused_indx];
            // first row
            println!(
                " {:vidd_1$} | {:vidd_2$} |",
                an_unused,
                file_list[0],
                vidd_1 = max_vidd_ref_unused,
                vidd_2 = max_vidd_file_unused
            );
            // rest of rows
            for rest_rows_index in 1..file_list.len() {
                println!(
                    " {:vidd_1$} | {:vidd_2$} |",
                    "",
                    file_list[rest_rows_index],
                    vidd_1 = max_vidd_ref_unused,
                    vidd_2 = max_vidd_file_unused
                );
            }
        }
    }
    /*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
    fn show_ambig(&mut self, out_ambig: &mut Vec<(String, Vec<String>, Vec<String>)>) {
        let mut max_vidd_ref_ambig: usize = "Symbol".len();
        let mut max_vidd_ref_file_ambig: usize = "Referenced in file(s)".len();
        let mut max_vidd_def_ambig: usize = "Is defined in file(s)".len();
        // calc column widths
        for an_ambig_indx in 0..out_ambig.len() {
            let an_ambig = &out_ambig[an_ambig_indx];
            if an_ambig.0.len() > max_vidd_ref_ambig {
                max_vidd_ref_ambig = an_ambig.0.len();
            }
            // column 2 (Referenced in file(s))
            let max_vidd_these_file_undefs = 0 +
                an_ambig.1.iter().map(|x| x.len()).fold(
                    0,
                    |max_sofar, i| if i >
                        max_sofar
                    {
                        i
                    } else {
                        max_sofar
                    },
                );
            if max_vidd_these_file_undefs > max_vidd_ref_file_ambig {
                max_vidd_ref_file_ambig = max_vidd_these_file_undefs;
            }
            // column 3 (Is defined in file(s))
            let max_vidd_these_file_undefs = 0 +
                an_ambig.2.iter().map(|x| x.len()).fold(
                    0,
                    |max_sofar, i| if i >
                        max_sofar
                    {
                        i
                    } else {
                        max_sofar
                    },
                );
            if max_vidd_these_file_undefs > max_vidd_def_ambig {
                max_vidd_def_ambig = max_vidd_these_file_undefs;
            }
        }
        // and print
        println!(
            "\nAmbiguous  definitions:
======================"
        );
        println!(
            " {:vidd_1$} | {:vidd_2$} | {:vidd_3$} |",
            "Symbol",
            "Referenced in file(s)",
            "Is defined in file(s)",
            vidd_1 = max_vidd_ref_ambig,
            vidd_2 = max_vidd_ref_file_ambig,
            vidd_3 = max_vidd_def_ambig
        );
        println!(
            " +{:->vidd$}+",
            "-",
            vidd = max_vidd_ref_ambig + 3 + max_vidd_ref_file_ambig + 3 + max_vidd_def_ambig
        );
        for an_ambig_indx in 0..out_ambig.len() {
            let (ref an_ambig, ref ref_file_list, ref def_file_list) = out_ambig[an_ambig_indx];
            // first row
            println!(
                " {:vidd_1$} | {:vidd_2$} | {:vidd_3$} |",
                an_ambig,
                ref_file_list[0],
                " -".to_string(),
                vidd_1 = max_vidd_ref_ambig,
                vidd_2 = max_vidd_ref_file_ambig,
                vidd_3 = max_vidd_def_ambig
            );
            // rest of ref files
            for rest_rows_index in 1..ref_file_list.len() {
                println!(
                    " {:vidd_1$} | {:vidd_2$} | {:vidd_3$} |",
                    "".to_string(),
                    ref_file_list[rest_rows_index],
                    " -".to_string(),
                    vidd_1 = max_vidd_ref_ambig,
                    vidd_2 = max_vidd_ref_file_ambig,
                    vidd_3 = max_vidd_def_ambig
                );
            }
            // and all def files alone in third column
            for def_file_rows_index in 0..def_file_list.len() {
                println!(
                    " {:vidd_1$} | {:vidd_2$} | {:vidd_3$} |",
                    "".to_string(),
                    " -".to_string(),
                    def_file_list[def_file_rows_index],
                    vidd_1 = max_vidd_ref_ambig,
                    vidd_2 = max_vidd_ref_file_ambig,
                    vidd_3 = max_vidd_def_ambig
                );
            }
        }
        /*
      println!(
        " {:vidd_1$} | {:vidd_2$} |{:vidd_3$} |",
        def_col_value,
        out_ambig[item_index].1,
        out_ambig[item_index].2,
        vidd_1 = max_vidd_ref_ambig,
        vidd_2 = max_vidd_ref_file_ambig,
        vidd_3 = max_vidd_def_ambig
      );

*/

    }
}
