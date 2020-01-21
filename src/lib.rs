// check rot_edge m(=n_syms) vs n_cubies
// TODO use mask values instead of depending on example goal repr
#![allow(dead_code)]
extern crate alginate_cube;
extern crate bitvec;
extern crate colored;
extern crate lazy_static;
extern crate proc_macro;
extern crate quote;
extern crate rand;
extern crate syn;

use proc_macro::TokenStream;
use quote::{format_ident, quote};

use alginate_cube::Cube;
use std::collections::HashMap;

// let mut solver = Solver::new("D' R2 D2 L' U F R' D' U2 L' U2 B2 R L2 F2 L F2", prunetbl);
// solver.solve();

// TODO use real mask check instead of comparing with repr cube mask
struct MaskDef {
    name: String,
    edge_orient_mask: [Option<u8>; 12],
    edge_perm_mask: [Option<u8>; 12],
    corner_orient_mask: [Option<u8>; 8],
    corner_perm_mask: [Option<u8>; 8],
}

impl MaskDef {
    pub fn eodfdb() -> Self {
        let edge_orient_mask = [Some(0); 12];
        let mut edge_perm_mask = [None; 12];
        let corner_orient_mask = [None; 8];
        let corner_perm_mask = [None; 8];
        edge_perm_mask[9] = Some(9);
        edge_perm_mask[10] = Some(10);

        Self {
            name: "eodfdb".to_owned(),
            edge_orient_mask,
            edge_perm_mask,
            corner_orient_mask,
            corner_perm_mask,
        }
    }

    pub fn post_eo_ld_1x2x3() -> Self {
        let edge_orient_mask = [None; 12];
        let mut edge_perm_mask = [None; 12];
        let mut corner_orient_mask = [None; 8];
        let mut corner_perm_mask = [None; 8];

        // we are only using this for zz
        // edge_orient_mask[6] = Some(0);
        // edge_orient_mask[7] = Some(0);
        // edge_orient_mask[11] = Some(0);

        edge_perm_mask[6] = Some(6);
        edge_perm_mask[7] = Some(7);
        edge_perm_mask[11] = Some(11);

        corner_orient_mask[6] = Some(0);
        corner_orient_mask[7] = Some(0);

        corner_perm_mask[6] = Some(6);
        corner_perm_mask[7] = Some(7);

        Self {
            name: "ld_1x2x3".to_owned(),
            edge_orient_mask,
            edge_perm_mask,
            corner_orient_mask,
            corner_perm_mask,
        }
    }

    pub fn ld_1x2x3() -> Self {
        let mut edge_orient_mask = [None; 12];
        let mut edge_perm_mask = [None; 12];
        let mut corner_orient_mask = [None; 8];
        let mut corner_perm_mask = [None; 8];

        // we are only using this for zz
        edge_orient_mask[6] = Some(0);
        edge_orient_mask[7] = Some(0);
        edge_orient_mask[11] = Some(0);

        edge_perm_mask[6] = Some(6);
        edge_perm_mask[7] = Some(7);
        edge_perm_mask[11] = Some(11);

        corner_orient_mask[6] = Some(0);
        corner_orient_mask[7] = Some(0);

        corner_perm_mask[6] = Some(6);
        corner_perm_mask[7] = Some(7);

        Self {
            name: "ld_1x2x3".to_owned(),
            edge_orient_mask,
            edge_perm_mask,
            corner_orient_mask,
            corner_perm_mask,
        }
    }
    pub fn post_eo_rd_1x2x3() -> Self {
        let edge_orient_mask = [None; 12];
        let mut edge_perm_mask = [None; 12];
        let mut corner_orient_mask = [None; 8];
        let mut corner_perm_mask = [None; 8];

        // we are only using this for zz
        // edge_orient_mask[4] = Some(0);
        // edge_orient_mask[5] = Some(0);
        // edge_orient_mask[8] = Some(0);

        edge_perm_mask[4] = Some(4);
        edge_perm_mask[5] = Some(5);
        edge_perm_mask[8] = Some(8);

        corner_orient_mask[4] = Some(0);
        corner_orient_mask[5] = Some(0);

        corner_perm_mask[4] = Some(6);
        corner_perm_mask[5] = Some(7);

        Self {
            name: "rd_1x2x3".to_owned(),
            edge_orient_mask,
            edge_perm_mask,
            corner_orient_mask,
            corner_perm_mask,
        }
    }

    pub fn rd_1x2x3() -> Self {
        let mut edge_orient_mask = [None; 12];
        let mut edge_perm_mask = [None; 12];
        let mut corner_orient_mask = [None; 8];
        let mut corner_perm_mask = [None; 8];

        edge_orient_mask[4] = Some(0);
        edge_orient_mask[5] = Some(0);
        edge_orient_mask[8] = Some(0);

        edge_perm_mask[4] = Some(4);
        edge_perm_mask[5] = Some(5);
        edge_perm_mask[8] = Some(8);

        corner_orient_mask[4] = Some(0);
        corner_orient_mask[5] = Some(0);

        corner_perm_mask[4] = Some(6);
        corner_perm_mask[5] = Some(7);

        Self {
            name: "rd_1x2x3".to_owned(),
            edge_orient_mask,
            edge_perm_mask,
            corner_orient_mask,
            corner_perm_mask,
        }
    }

    pub fn coll() -> Self {
        let edge_orient_mask = [None; 12];
        let edge_perm_mask = [None; 12];
        let mut corner_orient_mask = [None; 8];
        let mut corner_perm_mask = [None; 8];

        corner_orient_mask[0] = Some(0);
        corner_orient_mask[1] = Some(0);
        corner_orient_mask[2] = Some(0);
        corner_orient_mask[3] = Some(0);

        corner_perm_mask[0] = Some(0);
        corner_perm_mask[1] = Some(1);
        corner_perm_mask[2] = Some(2);
        corner_perm_mask[3] = Some(3);

        Self {
            name: "coll".to_owned(),
            edge_orient_mask,
            edge_perm_mask,
            corner_orient_mask,
            corner_perm_mask,
        }
    }
}

struct PruneTableDef {
    name: String,
    /// The piece attrs that we store in this table
    partial_mask: String,
    /// A representative goal state
    goal: Cube,
}

impl PruneTableDef {
    pub fn eodfdb() -> Self {
        Self {
            name: "eodfdb".to_owned(),
            partial_mask: "eodfdb".to_owned(),
            goal: Cube::new(),
        }
    }

    pub fn ld_1x2x3() -> Self {
        Self {
            name: "ld_1x2x3".to_owned(),
            partial_mask: "ld_1x2x3".to_owned(),
            goal: Cube::new(),
        }
    }

    pub fn rd_1x2x3() -> Self {
        Self {
            name: "rd_1x2x3".to_owned(),
            partial_mask: "rd_1x2x3".to_owned(),
            goal: Cube::new(),
        }
    }

    pub fn coll() -> Self {
        Self {
            name: "coll".to_owned(),
            partial_mask: "coll".to_owned(),
            goal: Cube::new(),
        }
    }
}

struct StepDef {
    name: String,
    // The goal
    solved_mask: String,
    prune_tables: Vec<String>,
    disallowed_moves: Vec<u8>,
    goal: Cube,
}

impl StepDef {
    pub fn pure_eoline() -> Self {
        Self {
            name: "pure_eoline".to_owned(),
            solved_mask: "eodfdb".to_owned(),
            prune_tables: vec!["eodfdb".to_owned()],
            goal: Cube::new(),
            disallowed_moves: vec![],
        }
    }
    pub fn roux_first_block() -> Self {
        Self {
            name: "roux_first_block".to_owned(),
            solved_mask: "ld_1x2x3".to_owned(),
            prune_tables: vec!["ld_1x2x3".to_owned()],
            goal: Cube::new(),
            // disallowed_moves: vec![6,8,15,17],
            disallowed_moves: vec![],
        }
    }
    pub fn zz_first_block() -> Self {
        Self {
            name: "zz_first_block".to_owned(),
            solved_mask: "ld_1x2x3".to_owned(),
            prune_tables: vec!["ld_1x2x3".to_owned(), "eodfdb".to_owned()],
            goal: Cube::new(),
            // disallowed_moves: vec![6,8,15,17],
            disallowed_moves: vec![],
        }
    }
    pub fn roux_second_block() -> Self {
        Self {
            name: "roux_second_block".to_owned(),
            solved_mask: "rd_1x2x3".to_owned(),
            prune_tables: vec![
                "rd_1x2x3".to_owned(),
                "ld_1x2x3".to_owned(),
            ],
            goal: Cube::new(),
            // disallowed_moves: vec![6,8,15,17],
            disallowed_moves: vec![],
        }
    }
    pub fn zz_second_block() -> Self {
        Self {
            name: "zz_second_block".to_owned(),
            solved_mask: "rd_1x2x3".to_owned(),
            prune_tables: vec![
                "rd_1x2x3".to_owned(),
                "ld_1x2x3".to_owned(),
                "eodfdb".to_owned(),
            ],
            goal: Cube::new(),
            // disallowed_moves: vec![6,8,15,17],
            disallowed_moves: vec![],
        }
    }
    // technically any F2L-covering set of aux prune tables
    pub fn zz_coll() -> Self {
        Self {
            name: "zz_coll".to_owned(),
            solved_mask: "rd_1x2x3".to_owned(),
            prune_tables: vec![
                "coll".to_owned(),
                "rd_1x2x3".to_owned(),
                "ld_1x2x3".to_owned(),
                "eodfdb".to_owned(),
            ],
            goal: Cube::new(),
            // disallowed_moves: vec![6,8,15,17],
            disallowed_moves: vec![],
        }
    }
}

struct Method {
    name: String,
    prune_tables: HashMap<String, PruneTableDef>,
    steps: HashMap<String, StepDef>,
    masks: HashMap<String, MaskDef>,
}

impl Method {
    pub fn roux() -> Self {
        let mut steps = HashMap::new();
        steps.insert("roux_first_block".to_owned(), StepDef::roux_first_block());
        steps.insert("roux_second_block".to_owned(), StepDef::roux_second_block());
        // steps.insert("zz_coll".to_owned(), StepDef::zz_coll());

        let mut prune_tables = HashMap::new();
        prune_tables.insert("ld_1x2x3".to_owned(), PruneTableDef::ld_1x2x3());
        prune_tables.insert("rd_1x2x3".to_owned(), PruneTableDef::rd_1x2x3());
        // prune_tables.insert("coll".to_owned(), PruneTableDef::coll());

        let mut masks = HashMap::new();
        masks.insert("ld_1x2x3".to_owned(), MaskDef::ld_1x2x3());
        masks.insert("rd_1x2x3".to_owned(), MaskDef::rd_1x2x3());
        // masks.insert("coll".to_owned(), MaskDef::coll());

        Self {
            name: "roux".to_owned(),
            steps,
            prune_tables,
            masks,
        }
    }
    // FIXME fix post eo masks
    pub fn zz() -> Self {
        let mut steps = HashMap::new();
        steps.insert("pure_eoline".to_owned(), StepDef::pure_eoline());
        steps.insert("zz_first_block".to_owned(), StepDef::zz_first_block());
        steps.insert("zz_second_block".to_owned(), StepDef::zz_second_block());
        steps.insert("zz_coll".to_owned(), StepDef::zz_coll());

        let mut prune_tables = HashMap::new();
        prune_tables.insert("eodfdb".to_owned(), PruneTableDef::eodfdb());
        prune_tables.insert("ld_1x2x3".to_owned(), PruneTableDef::ld_1x2x3());
        prune_tables.insert("rd_1x2x3".to_owned(), PruneTableDef::rd_1x2x3());
        prune_tables.insert("coll".to_owned(), PruneTableDef::coll());

        let mut masks = HashMap::new();
        masks.insert("eodfdb".to_owned(), MaskDef::eodfdb());
        masks.insert("ld_1x2x3".to_owned(), MaskDef::ld_1x2x3());
        masks.insert("rd_1x2x3".to_owned(), MaskDef::rd_1x2x3());
        masks.insert("coll".to_owned(), MaskDef::coll());

        Self {
            name: "zz".to_owned(),
            steps,
            prune_tables,
            masks,
        }
    }
}

type QTokenStream = quote::__rt::TokenStream;

fn gs_mask_fn(mask_name: &str) -> syn::Ident {
    format_ident!("get_{}_submodel", mask_name)
}

fn gs_table_field(table_name: &str) -> syn::Ident {
    format_ident!("{}_table", table_name)
}

fn gs_table_goal_field(table_name: &str) -> syn::Ident {
    format_ident!("{}_goal", table_name)
}

fn gs_table_rc(table_name: &str) -> syn::Ident {
    format_ident!("{}_table", table_name)
}

fn gs_table(table_name: &str) -> syn::Ident {
    format_ident!("{}", table_name)
}

fn gs_gen_table_fn(table_name: &str) -> syn::Ident {
    format_ident!("generate_{}_prune_table", table_name)
}

#[proc_macro]
pub fn gen_solver(input: TokenStream) -> TokenStream {
    let _ = input;
    let tokens = mk_method_solver(&Method::zz());
    TokenStream::from(tokens)
}

fn mk_method_solver(method: &Method) -> QTokenStream {
    let method_name = format_ident!("{}", method.name);
    let greeting = format!("{} solver generated by alginate", method.name);

    let mask_getters = method
        .masks
        .values()
        .map(|mask| mk_mask_getter(method, mask))
        .collect::<Vec<_>>();
    let prunetbl_generators = method
        .prune_tables
        .values()
        .map(|prune_table| mk_prunetbl_generator(method, prune_table))
        .collect::<Vec<_>>();
    let step_solvers = method
        .steps
        .values()
        .map(|step| mk_step_solver(method, step))
        .collect::<Vec<_>>();

    let mut gen_prune_table_rcs = Vec::new();

    for (prune_table_name, _prune_table) in method.prune_tables.iter() {
        let table = gs_table(prune_table_name);
        let table_rc = gs_table_rc(prune_table_name);

        gen_prune_table_rcs.push(quote! {
            let #table_rc = ::std::rc::Rc::new(prune::#table::generate());
        });
    }

    let tokens = quote! {
        pub mod #method_name {
            pub mod mask {
                #(#mask_getters)*
            }
            pub mod prune {
                #(#prunetbl_generators)*
            }
            pub mod step {
                #(#step_solvers)*
            }
            pub fn toplevel() {
                println!(#greeting);
                let _ = "generate prune tables";
                #(#gen_prune_table_rcs)*
                let _ = "steps";
                let mut scramble = "B2 L2 B D2 R2 F' L2 F U2 L2 F R' F' D L U B R B2 F2 R";

                // let solution = Vec::new();
                // let mut roux_first_block_solver = step::roux_first_block::Solver::new(
                //         scramble, &solution.clone(), &ld_1x2x3_table);
                // let fb_solutions = roux_first_block_solver.solve();
                // for fb_solution in fb_solutions.into_iter() {
                //     for mov in fb_solution.iter() {
                //         print!("{} ", mov);
                //     }
                //     println!();

                //     let mut solution2 = solution.clone();
                //     solution2.extend(&fb_solution);
                //     let mut sb_solver = step::roux_second_block::Solver::new(
                //             scramble, &solution2.clone(), &rd_1x2x3_table, &ld_1x2x3_table);
                //     let sb_solutions = sb_solver.solve();

                //     for sb_solution in sb_solutions.into_iter() {
                //         print!("    ");
                //         for mov in sb_solution.iter() {
                //             print!("{} ", mov);
                //         }
                //         println!();
                //     }
                // }

                let solution = Vec::new();
                let mut pure_eoline_solver = step::pure_eoline::Solver::new(
                        scramble, &solution.clone(), &eodfdb_table);
                let eoline_solutions = pure_eoline_solver.solve();
                for eoline_solution in eoline_solutions.into_iter() {
                    for mov in eoline_solution.iter() {
                        print!("{} ", mov);
                    }
                    println!();

                    let mut solution2 = solution.clone();
                    solution2.extend(&eoline_solution);
                    let mut fb_solver = step::zz_first_block::Solver::new(
                            scramble, &solution2.clone(), &ld_1x2x3_table, &eodfdb_table);
                    let fb_solutions = fb_solver.solve();

                    for fb_solution in fb_solutions.into_iter() {
                        print!("    ");
                        for mov in fb_solution.iter() {
                            print!("{} ", mov);
                        }
                        println!();
                        let mut solution3 = solution2.clone();
                        solution3.extend(&fb_solution);

                        let mut sb_solver = step::zz_second_block::Solver::new(
                                scramble, &solution3.clone(), 
                                &rd_1x2x3_table, &ld_1x2x3_table, &eodfdb_table);
                        let sb_solutions = sb_solver.solve();
                        for sb_solution in sb_solutions.into_iter() {
                            print!("        ");
                            for mov in sb_solution.iter() {
                                print!("{} ", mov);
                            }
                            println!();
                            // let mut solution4 = solution3.clone();
                            // solution4.extend(&fb_solution);

                            // let mut coll_solver = step::zz_coll::Solver::new(
                            //         scramble, &solution4.clone(), 
                            //         &coll_table, &rd_1x2x3_table, &ld_1x2x3_table, &eodfdb_table);
                            // let coll_solutions = coll_solver.solve();
                            // for coll_solution in coll_solutions.into_iter() {
                            //     print!("        ");
                            //     for mov in coll_solution.iter() {
                            //         print!("{} ", mov);
                            //     }
                            //     println!();
                            // }
                        }
                    }
                }
            }
        }
    };
    tokens
}

fn mk_mask_getter(_method: &Method, mask: &MaskDef) -> QTokenStream {
    let fn_name = gs_mask_fn(&mask.name);
    let mut bit_counter: usize;

    bit_counter = 0;
    let mut store_edge_orients = Vec::new();
    for (ix, v) in mask.edge_orient_mask.iter().enumerate() {
        if let Some(_) = v {
            // one bit wide
            let final_bit = bit_counter + 1 - 1;
            store_edge_orients.push(quote! {
                ret &= !(1 << #bit_counter);
                ret |= (cube.edges[#ix].orient().0 as u64) << #bit_counter;
            });
            bit_counter = final_bit + 1;
        }
    }

    let mut store_corner_orients = Vec::new();
    for (ix, v) in mask.corner_orient_mask.iter().enumerate() {
        if let Some(_) = v {
            let final_bit = bit_counter + 2 - 1;
            store_corner_orients.push(quote! {
                ret &= !(11 << #bit_counter);
                ret |= (cube.corns[#ix].orient().0 as u64) << #bit_counter;
            });
            bit_counter = final_bit + 1;
        }
    }

    let mut store_edge_perms = Vec::new();
    for (ix, v) in mask.edge_perm_mask.iter().enumerate() {
        if let Some(_) = v {
            // four bits wide
            let final_bit = bit_counter + 4 - 1;
            store_edge_perms.push(quote! {
                ret &= !(1111 << #bit_counter);
                ret |= (cube.edges[#ix].perm().0 as u64) << #bit_counter;
            });
            bit_counter = final_bit + 1;
        }
    }

    let mut store_corner_perms = Vec::new();
    for (ix, v) in mask.corner_perm_mask.iter().enumerate() {
        if let Some(_) = v {
            // three bits wide
            let final_bit = bit_counter + 3 - 1;
            store_corner_perms.push(quote! {
                ret &= !(111 << #bit_counter);
                ret |= (cube.corns[#ix].perm().0 as u64) << #bit_counter;
            });
            bit_counter = final_bit + 1;
        }
    }

    quote! {
        pub fn #fn_name(cube: &::alginate_cube::Cube) -> ::alginate_cube::Submodel {
            let mut ret = 0;
            let _ = "storing edge orients";
            #(#store_edge_orients)*
            let _ = "storing corner orients";
            #(#store_corner_orients)*
            let _ = "storing edge perms";
            #(#store_edge_perms)*
            let _ = "storing corner perms";
            #(#store_corner_perms)*
            ret
        }
    }
}

fn mk_prunetbl_generator(_method: &Method, tbl: &PruneTableDef) -> QTokenStream {
    let table = gs_table(&tbl.name);
    let mask_fn = gs_mask_fn(&tbl.partial_mask);
    quote! {
        pub mod #table {
            pub struct Table(pub ::alginate_cube::PruneTable);
            pub fn generate() -> Table {
                Table(::alginate_cube::generate_prune_table(super::super::mask::#mask_fn))
            }
        }
    }
}

fn mk_step_solver(method: &Method, step: &StepDef) -> QTokenStream {
    let mod_name = format_ident!("{}", step.name);

    let mut table_field_names = Vec::new();
    let mut table_fields = Vec::new();
    let mut table_field_refs = Vec::new();
    let mut goal_field_names = Vec::new();
    let mut goal_field_initialisers = Vec::new();
    let mut max_prunes = Vec::new();
    let mut check_goals = Vec::new();

    for prune_table in step.prune_tables.iter() {
        let table = gs_table(&prune_table);
        let table_field = gs_table_field(&prune_table);
        let goal_field = gs_table_goal_field(&prune_table);
        let goal_submodel_fn = gs_mask_fn(&method.prune_tables[prune_table].partial_mask);
        table_field_names.push(table_field.clone());
        goal_field_names.push(goal_field.clone());
        goal_field_initialisers.push(quote! {
            let #goal_field = super::super::mask::#goal_submodel_fn(&state);
        });

        table_fields.push(quote! {
            #table_field: ::std::rc::Rc<super::super::prune::#table::Table>
        });

        table_field_refs.push(quote! {
            #table_field: &::std::rc::Rc<super::super::prune::#table::Table>
        });

        max_prunes.push(quote! {
            let submodel = super::super::mask::#goal_submodel_fn(&self.state);
            if let Some(depth_bound)
                = self.#table_field.0.get(&submodel) {
                if *depth_bound > max {
                    max = *depth_bound;
                }
            }
        });

        check_goals.push(quote! {
            if super::super::mask::#goal_submodel_fn(&self.state) != self.#goal_field {
                return false;
            }
        });
    }

    let solver_struct = quote! {
        pub struct Solver {
            state: ::alginate_cube::Cube,
            start_depth: ::alginate_cube::Depth,
            max_depth: ::alginate_cube::Depth,
            slack: u32,
            max_solns: u32,
            soln: ::std::vec::Vec<::alginate_cube::Move>,
            solns: ::std::vec::Vec<::std::vec::Vec<::alginate_cube::Move>>,
            num_nodes: u32,
            num_solns: u32,
            prune_table_hits: u32,
            min_soln_len: u64,
            #(#table_fields),*,
            #(#goal_field_names: ::alginate_cube::Submodel),*,
        }
    };

    let mut solver_impl_fns = Vec::new();

    solver_impl_fns.push(quote! {
        pub fn new(
            scramble: &str,
            prev_solutions: &::std::vec::Vec<::alginate_cube::Move>,
            #(#table_field_refs),*,)
        -> Self {
            let mut state = ::alginate_cube::Cube::new();
            #(#goal_field_initialisers)*

            state.scramble(scramble);
            state.scramble_vec(prev_solutions);

            // goal_field_names
            Self {
                state,
                start_depth: 1,
                max_depth: 999,
                slack: 0,
                max_solns: 100,
                soln: Vec::new(),
                solns: Vec::new(),
                num_nodes: 0,
                num_solns: 0,
                prune_table_hits: 0,
                min_soln_len: 999,
                #(#table_field_names: ::std::rc::Rc::clone(#table_field_names)),*,
                #(#goal_field_names),*,
            }
        }
    });

    solver_impl_fns.push(quote! {
        fn prune(&self, depth: ::alginate_cube::Depth) -> bool {
            let mut max = 0;

            #(#max_prunes)*

            if max > depth {
                return true;
            }
            false
        }
    });

    solver_impl_fns.push(quote! {
        fn check_goal(&mut self) -> bool {
            // print!("checking goal");
            #(#check_goals)*

            // for mov in self.soln.iter() {
            //     print!("{} ", mov);
            // }
            // println!();
            if (self.soln.len() as u64) < self.min_soln_len {
                self.min_soln_len = self.soln.len() as u64;
                self.solns.push(self.soln.clone());
                self.num_solns += 1;
                true
            } else {
                false
            }
        }
    });

    solver_impl_fns.push(quote! {
        pub fn solve(&mut self) -> Vec<Vec<::alginate_cube::Move>> {
            let mut solved: bool = false;
            let mut slack_counter = 0;
            let mut last_iter_nodes = 1;

            // if !self.eodfdb_table.0.contains_key(
            //     &super::super::mask::get_eodfdb_submodel(&self.state)) {
            //     eprintln!("possibly unsolvable eodfdb state");
            // }

            for i in self.start_depth ..= self.max_depth {
                let mut this_iter_nodes = self.num_nodes;
                if solved && slack_counter == self.slack {
                    break;
                }
                // f::start(format!("search depth {}", i));

                if solved {
                    slack_counter += 1;
                }

                if self.search(i, None) {
                    solved = true;
                }

                this_iter_nodes = self.num_nodes - this_iter_nodes;
                last_iter_nodes = this_iter_nodes;
                // println!("at depth {} #solns = {}, slack {}, table hits {}",
                //          i, self.num_solns, self.slack,
                //          self.prune_table_hits);
            }

            self.solns.clone()
        }
    });

    let mut disallowed_move_check = Vec::new();

    for raw_mov in step.disallowed_moves.iter() {
        disallowed_move_check.push(quote! {
            if mov_ == #raw_mov {
                continue;
            }
        });
    }

    solver_impl_fns.push(quote! {
        fn search(&mut self, depth: u32, prev_face: Option<u8>) -> bool {
            // print!("\ndepth {} | ", depth);
            // for mov in self.soln.iter() {
            //     print!("{} ", mov);
            // }

            if self.check_goal() {
                return true;
            }

            if let Some(_pf) = prev_face {
                // print!("prevface {} " , pf);
                if self.num_solns == self.max_solns {
                    return true;
                }

                if depth == 0 {
                    return self.check_goal();
                }
            }

            if self.prune(depth) {
                // print!("pruned");
                self.prune_table_hits += 1;
                return false;
            }

            let mut s = false;
            for face in 0..::alginate_cube::N_FACES as u8 {
                if prev_face == Some(face) {
                    continue;
                }
                // TODO face + 3 for R/L etc
                let base_mov = ::alginate_cube::Move::new(3 * face);
                for turns in 0..3 {
                    let mov_ = 3 * face + turns;
                    // #(#disallowed_move_check)*
                    let mov = ::alginate_cube::Move::new(mov_);
                    self.state.apply_move(base_mov);
                    self.soln.push(mov);
                    self.num_nodes += 1;
                    if self.search(depth - 1, Some(face)) {
                        s = true;
                    }
                    self.soln.pop();
                }
                // assume all turns are order 4
                self.state.apply_move(base_mov);
            }

            s
        }
    });

    quote! {
        pub mod #mod_name {
            #solver_struct

            impl Solver {
                #(#solver_impl_fns)*
            }
        }
    }
}
