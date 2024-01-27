use std::collections::BTreeMap;

use indexmap::IndexSet;

use crate::{
  term::{Book, DefId, Definition, Term},
  Warning,
};

type Definitions = IndexSet<DefId>;

impl Book {
  /// Removes all unused definitions starting from Main.
  pub fn prune(&mut self, main: DefId, prune: bool, warnings: &mut Vec<Warning>) {
    let mut used = Definitions::new();
    // Since the readback needs the constructors to work we need
    // to mark as used all constructors in case of the user
    // manually encode a constructor with tagged lambdas
    for ctr in self.ctrs.keys() {
      let def = self.defs.get(&self.def_names.name_to_id[ctr]).unwrap();
      used.insert(def.def_id);
      def.assert_no_pattern_matching_rules();
      def.rules[0].body.find_used_definitions(&mut used, &self.defs);
    }

    let def = self.defs.get(&main).unwrap();
    used.insert(def.def_id);
    def.rules[0].body.find_used_definitions(&mut used, &self.defs);

    let ids = IndexSet::<DefId>::from_iter(self.def_names.def_ids().copied());
    let unused = ids.difference(&used);
    for &unused_id in unused {
      if prune {
        self.remove_def(unused_id);
      } else if !self.is_def_name_generated(unused_id) {
        let def_name = self.def_names.id_to_name[&unused_id].clone();
        warnings.push(Warning::UnusedDefinition { def_name })
      }
    }
  }
}

impl Term {
  /// Finds all used definitions on every term that can have a def_id.
  fn find_used_definitions(&self, used: &mut Definitions, defs: &BTreeMap<DefId, Definition>) {
    let mut to_visit = vec![self];

    while let Some(term) = to_visit.pop() {
      match term {
        Term::Ref { def_id } => {
          if used.insert(*def_id) {
            let Definition { rules, .. } = defs.get(def_id).unwrap();
            for rule in rules {
              to_visit.push(&rule.body);
            }
          }
        }
        Term::Lam { bod, .. } | Term::Chn { bod, .. } => bod.find_used_definitions(used, defs),
        Term::Let { val, nxt, .. } | Term::Dup { val, nxt, .. } => {
          val.find_used_definitions(used, defs);
          nxt.find_used_definitions(used, defs);
        }
        Term::App { fun, arg, .. } => {
          fun.find_used_definitions(used, defs);
          arg.find_used_definitions(used, defs);
        }
        Term::Sup { fst, snd, .. } | Term::Tup { fst, snd } | Term::Opx { fst, snd, .. } => {
          fst.find_used_definitions(used, defs);
          snd.find_used_definitions(used, defs);
        }
        Term::Match { scrutinee, arms } => {
          scrutinee.find_used_definitions(used, defs);
          for (_, term) in arms {
            term.find_used_definitions(used, defs);
          }
        }
        Term::Var { .. }
        | Term::Lnk { .. }
        | Term::Num { .. }
        | Term::Str { .. }
        | Term::List { .. }
        | Term::Era => (),
      }
    }
  }
}
