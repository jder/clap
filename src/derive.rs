//! This module contains traits that are usable with the `#[derive(...)].`
//! macros in [`clap_derive`].

use crate::{App, ArgMatches, Error};

use std::ffi::OsString;

/// The primary one-stop-shop trait used to create an instance of a `clap`
/// [`App`], conduct the parsing, and turn the resulting [`ArgMatches`] back
/// into concrete instance of the user struct.
///
/// This trait is primarily A convenience on top of [`FromArgMatches`] +
/// [`IntoApp`] which uses those two underlying traits to build the two
/// fundamental functions `parse` which uses the `std::env::args_os` iterator,
/// and `parse_from` which allows the consumer to supply the iterator (along
/// with fallible options for each).
///
/// # Examples
///
/// The following example creates a `Context` struct that would be used
/// throughout the application representing the normalized values coming from
/// the CLI.
///
/// ```rust
/// # use clap::{Clap};
/// /// My super CLI
/// #[derive(Clap)]
/// #[clap(name = "demo")]
/// struct Context {
///     /// More verbose output
///     #[clap(long)]
///     verbose: bool,
///     /// An optional name
///     #[clap(short, long)]
///     name: Option<String>,
/// }
/// ```
///
/// The equivilant [`App`] struct + `From` implementation:
///
/// ```rust
/// # use clap::{App, Arg, ArgMatches};
/// App::new("demo")
///     .about("My super CLI")
///     .arg(Arg::with_name("verbose")
///         .long("verbose")
///         .help("More verbose output"))
///     .arg(Arg::with_name("name")
///         .long("name")
///         .long("n")
///         .help("An optional name")
///         .takes_value(true));
///
/// struct Context {
///     verbose: bool,
///     name: Option<String>,
/// }
///
/// impl From<ArgMatches> for Context {
///     fn from(m: ArgMatches) -> Self {
///         Context {
///             verbose: m.is_present("verbose"),
///             name: m.value_of("name").map(|n| n.to_owned()),
///         }
///     }
/// }
/// ```
pub trait Clap: FromArgMatches + IntoApp + Sized {
    /// Parse from `std::env::args_os()`, exit on error
    fn parse() -> Self {
        let matches = <Self as IntoApp>::into_app().get_matches();
        <Self as FromArgMatches>::from_arg_matches(&matches)
    }

    /// Parse from `std::env::args_os()`, return Err on error.
    fn try_parse() -> Result<Self, Error> {
        let matches = <Self as IntoApp>::into_app().try_get_matches()?;
        Ok(<Self as FromArgMatches>::from_arg_matches(&matches))
    }

    /// Parse from iterator, exit on error
    fn parse_from<I, T>(itr: I) -> Self
    where
        I: IntoIterator<Item = T>,
        // TODO (@CreepySkeleton): discover a way to avoid cloning here
        T: Into<OsString> + Clone,
    {
        let matches = <Self as IntoApp>::into_app().get_matches_from(itr);
        <Self as FromArgMatches>::from_arg_matches(&matches)
    }

    /// Parse from iterator, return Err on error.
    fn try_parse_from<I, T>(itr: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = T>,
        // TODO (@CreepySkeleton): discover a way to avoid cloning here
        T: Into<OsString> + Clone,
    {
        let matches = <Self as IntoApp>::into_app().try_get_matches_from(itr)?;
        Ok(<Self as FromArgMatches>::from_arg_matches(&matches))
    }
}

/// Build an App according to the struct
///
/// Also serves for flattening
pub trait IntoApp: Sized {
    /// @TODO @release @docs
    fn into_app<'help>() -> App<'help>;
    /// @TODO @release @docs
    fn augment_clap(app: App<'_>) -> App<'_>;
}

/// Extract values from ArgMatches into the struct.
pub trait FromArgMatches: Sized {
    /// @TODO @release @docs
    fn from_arg_matches(matches: &ArgMatches) -> Self;
}

/// @TODO @release @docs
pub trait Subcommand: Sized {
    /// @TODO @release @docs
    fn from_subcommand(subcommand: Option<(&str, &ArgMatches)>) -> Option<Self>;
    /// @TODO @release @docs
    fn augment_subcommands(app: App<'_>) -> App<'_>;
}

/// @TODO @release @docs
pub trait ArgEnum: Sized {
    /// @TODO @release @docs
    const VARIANTS: &'static [&'static str];

    /// @TODO @release @docs
    fn from_str(input: &str, case_insensitive: bool) -> Result<Self, String>;
}

impl<T: Clap> Clap for Box<T> {
    fn parse() -> Self {
        Box::new(<T as Clap>::parse())
    }

    fn try_parse() -> Result<Self, Error> {
        <T as Clap>::try_parse().map(Box::new)
    }

    fn parse_from<I, It>(itr: I) -> Self
    where
        I: IntoIterator<Item = It>,
        // TODO (@CreepySkeleton): discover a way to avoid cloning here
        It: Into<OsString> + Clone,
    {
        Box::new(<T as Clap>::parse_from(itr))
    }

    fn try_parse_from<I, It>(itr: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = It>,
        // TODO (@CreepySkeleton): discover a way to avoid cloning here
        It: Into<OsString> + Clone,
    {
        <T as Clap>::try_parse_from(itr).map(Box::new)
    }
}

impl<T: IntoApp> IntoApp for Box<T> {
    fn into_app<'help>() -> App<'help> {
        <T as IntoApp>::into_app()
    }
    fn augment_clap(app: App<'_>) -> App<'_> {
        <T as IntoApp>::augment_clap(app)
    }
}

impl<T: FromArgMatches> FromArgMatches for Box<T> {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        Box::new(<T as FromArgMatches>::from_arg_matches(matches))
    }
}

impl<T: Subcommand> Subcommand for Box<T> {
    fn from_subcommand(subcommand: Option<(&str, &ArgMatches)>) -> Option<Self> {
        <T as Subcommand>::from_subcommand(subcommand).map(Box::new)
    }
    fn augment_subcommands(app: App<'_>) -> App<'_> {
        <T as Subcommand>::augment_subcommands(app)
    }
}
