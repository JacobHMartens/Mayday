use std::env;

use crate::collector::UnsafeCollector;

pub trait UnsafeReporter {
    fn report_unsafe_code(&self);
}

impl<'hir> UnsafeReporter for UnsafeCollector<'hir> {
    fn report_unsafe_code(&self) {
        let crate_name = env::args().skip_while(|arg| arg != "--crate-name").nth(1).unwrap();
        let analyse_crate_string = format!("Analysing crate: {}", crate_name);
        let hline = "-".repeat(analyse_crate_string.len());
        println!("\n{}\n{}", analyse_crate_string, hline);
        let unsafe_fn_percentage =  calc_percentage(self.unsafe_functions.len() as f32, self.count_functions as f32);
        println!("Unsafe functions: {:?} out of {:?} functions ({:.1}%)", self.unsafe_functions.len(), self.count_functions, unsafe_fn_percentage);

        let unsafe_trait_percentage =  calc_percentage(self.unsafe_traits.len() as f32, self.count_traits as f32);
        println!("Unsafe Traits: {:?} out of {:?} traits ({:.1}%)", self.unsafe_traits.len(), self.count_traits, unsafe_trait_percentage);

        let unsafe_impl_percentage =  calc_percentage(self.unsafe_impls.len() as f32, self.count_impls as f32);
        println!("Unsafe Impls: {:?} out of {:?} trait implementations ({:.1}%)", self.unsafe_impls.len(), self.count_impls, unsafe_impl_percentage);

        let unsafe_block_percentage =  calc_percentage(self.unsafe_blocks.len() as f32, self.count_blocks as f32);
        println!("Unsafe Blocks: {:?} out of {:?} blocks ({:.1}%)", self.unsafe_blocks.len(), self.count_blocks, unsafe_block_percentage);
        println!();
    }
}

fn calc_percentage(count: f32, total: f32) -> f32 {
    if total == 0.0 { return 0.0 }
    else {
        return 100.0 * count/total;
    }
}