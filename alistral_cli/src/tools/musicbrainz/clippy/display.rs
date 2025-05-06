use core::fmt::Write as _;

use alistral_core::cli::colors::AlistralColors as _;
use strum_macros::Display;
use strum_macros::EnumIter;
use symphonize::clippy::lint_result::LintResult;
use tuillez::OwoColorize as _;
use tuillez::extensions::inquire_ext::select_enum::select_enum;

use crate::ALISTRAL_CLIENT;

/// Actions to take after displaying a lint
#[derive(Debug, EnumIter, Display)]
pub(super) enum LintActions {
    Done,
    //Skip,
    Exit,
}

pub(super) async fn print_lint<L: LintResult>(lint: &L) -> LintActions {
    println!("{}", format_lint(lint).await);

    select_enum::<LintActions>("").prompt().unwrap()
}

pub(super) async fn format_lint<L: LintResult>(lint: &L) -> String {
    let mut report = String::new();
    writeln!(&mut report).unwrap();
    writeln!(
        &mut report,
        "{}",
        format!("\n {} ", L::get_name())
            .on_truecolor_tup(lint.get_severity().get_color())
            .black()
            .bold()
    )
    .unwrap();
    writeln!(&mut report).unwrap();
    writeln!(
        &mut report,
        "{}",
        lint.get_body(&ALISTRAL_CLIENT.symphonize)
            .await
            .expect("Error while processing lint body")
    )
    .unwrap();

    // Hints
    let hints = lint
        .get_hints(&ALISTRAL_CLIENT.symphonize)
        .await
        .expect("Error while processing lint hints");
    if !hints.is_empty() {
        writeln!(&mut report).unwrap();
        for hint in hints {
            writeln!(&mut report, "{hint}").unwrap();
        }
    }

    // Links
    writeln!(&mut report).unwrap();
    writeln!(&mut report, "Links:").unwrap();
    for link in lint
        .get_links(&ALISTRAL_CLIENT.symphonize)
        .await
        .expect("Error while processing lint links")
    {
        writeln!(&mut report, "    - {link}").unwrap();
    }

    writeln!(&mut report).unwrap();
    report
}
