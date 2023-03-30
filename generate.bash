#!/usr/bin/env bash
cp src/visit_mut.rs src/visit.rs # base copy
sd 'VisitMut' 'Visit' src/visit.rs # rename traits
sd 'as_mut' 'as_ref' src/visit.rs # Option::as_mut -> Option::as_ref
sd '_mut\(' '(' src/visit.rs # rename trait functions
sd '_mut<' '<' src/visit.rs # rename top level functions
sd "&'openapi mut" "&'openapi" src/visit.rs # change reference mutability
cargo fmt
