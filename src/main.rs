use crate::align::{ReadAlignment, ReadAlignmentError};
use crate::cli::Cli;
use anyhow::Result;
use structopt::StructOpt;

mod align;
mod cli;
mod covplot;

/// Vircov application
///
/// Run the application from arguments provided
/// by the command line interface
#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), ReadAlignmentError> {
    let args = Cli::from_args();

    let verbose = match args.group_select_split {
        Some(_) => 2, // for group refseq selection we need the tags
        None => args.verbose,
    };

    let mut align = ReadAlignment::new(&args.fasta, &args.exclude)?;

    let align = align.from(
        args.alignment,
        args.min_len,
        args.min_cov,
        args.min_mapq,
        args.alignment_format,
    )?;

    let data = align.coverage_statistics(
        args.regions,
        args.seq_len,
        args.coverage,
        args.reads,
        &args.group_by,
        verbose,
    )?;

    match args.group_by {
        None => {
            align.to_output(
                &data,
                args.table,
                args.read_ids,
                args.read_ids_split,
                None,
                None,
            )?;
        }
        Some(group_field) => {
            match align.target_sequences {
                None => return Err(ReadAlignmentError::GroupSequenceError()),
                Some(_) => {
                    match args.covplot {
                        true => return Err(ReadAlignmentError::GroupCovPlotError()),
                        false => {
                            // If reference sequences have been provided, continue with grouping outputs
                            let grouped_data = align.group_output(
                                &data,
                                args.group_regions,
                                args.group_coverage,
                                group_field,
                                args.group_sep,
                            )?;
                            align.to_output(
                                &grouped_data,
                                args.table,
                                args.read_ids,
                                args.read_ids_split,
                                args.group_select_by,
                                args.group_select_split,
                            )?;
                        }
                    };
                }
            }
        }
    };

    match args.covplot {
        true => align.coverage_plots(&data, args.width)?,
        false => {}
    }

    Ok(())
}
