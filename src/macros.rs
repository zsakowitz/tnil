//! Provides macros to aid in constructing words.

#[macro_export]
/// Creates a [`NormalRelation`][crate::word::formative::relation::NormalRelation] using more
/// concise syntax that writing out the enum and its fields in full.
///
/// # Examples
///
/// ```
/// # use tnil::relation;
/// # use tnil::word::formative::relation::NormalRelation;
/// # use tnil::category::{Case, CaseScope, IllocutionOrValidation, Mood, NominalMode};
/// # fn main() {
/// let default_relation = relation!(NOM, CCN, THM);
///
/// assert_eq!(default_relation, NormalRelation::Nominal {
///     mode: NominalMode::NOM,
///     case_scope: CaseScope::CCN,
///     case: Case::THM,
/// });
///
/// let default_verbal_relation = relation!(FAC, OBS);
///
/// assert_eq!(default_verbal_relation, NormalRelation::Verbal {
///     mood: Mood::FAC,
///     ivl: IllocutionOrValidation::OBS,
/// });
///
/// let t1_ccq_all = relation!(T1, CCQ, ALL);
///
/// assert_eq!(t1_ccq_all, NormalRelation::Nominal {
///     mode: NominalMode::T1,
///     case_scope: CaseScope::CCQ,
///     case: Case::ALL,
/// });
/// # }
/// ```
macro_rules! relation {
    ($mode:ident, $case_scope:ident, $case:ident) => {
        $crate::word::formative::relation::NormalRelation::Nominal {
            mode: $crate::category::NominalMode::$mode,
            case_scope: $crate::category::CaseScope::$case_scope,
            case: $crate::category::Case::$case,
        }
    };

    ($mood:ident, $ivl:ident) => {
        $crate::word::formative::relation::NormalRelation::Verbal {
            mood: $crate::category::Mood::$mood,
            ivl: $crate::category::IllocutionOrValidation::$ivl,
        }
    };
}

#[macro_export]
/// Creates a [`Ca`][crate::category::Ca] using more concise syntax that writing it out in full.
/// Also works in `const` contexts.
///
/// # Examples
///
/// ```
/// # use tnil::ca;
/// # use tnil::category::{Ca, Affiliation, Configuration, Extension, Perspective, Essence};
/// # fn main() {
/// let short = ca!(COA, G, RPV);
///
/// let long = Ca {
///     affiliation: Affiliation::COA,
///     perspective: Perspective::G,
///     essence: Essence::RPV,
///     ..Default::default()
/// };
///
/// assert_eq!(short, long);
///
/// const AGGLOMERATIVE_CA: Ca = ca!(G);
/// # }
/// ```
macro_rules! ca {
    (@ CSL $($x:ident)*) => { ca!($crate::category::Affiliation::CSL, @ $($x)*) };
    (@ ASO $($x:ident)*) => { ca!($crate::category::Affiliation::ASO, @ $($x)*) };
    (@ COA $($x:ident)*) => { ca!($crate::category::Affiliation::COA, @ $($x)*) };
    (@ VAR $($x:ident)*) => { ca!($crate::category::Affiliation::VAR, @ $($x)*) };
    (@     $($x:ident)*) => { ca!($crate::category::Affiliation::CSL, @ $($x)*) };

    ($affiliation:expr, @ UPX $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::UPX, @ $($x)*) };
    ($affiliation:expr, @ MSS $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::MSS, @ $($x)*) };
    ($affiliation:expr, @ MSC $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::MSC, @ $($x)*) };
    ($affiliation:expr, @ MSF $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::MSF, @ $($x)*) };
    ($affiliation:expr, @ MDS $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::MDS, @ $($x)*) };
    ($affiliation:expr, @ MDC $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::MDC, @ $($x)*) };
    ($affiliation:expr, @ MDF $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::MDF, @ $($x)*) };
    ($affiliation:expr, @ MFS $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::MFS, @ $($x)*) };
    ($affiliation:expr, @ MFC $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::MFC, @ $($x)*) };
    ($affiliation:expr, @ MFF $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::MFF, @ $($x)*) };
    ($affiliation:expr, @ DPX $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::DPX, @ $($x)*) };
    ($affiliation:expr, @ DSS $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::DSS, @ $($x)*) };
    ($affiliation:expr, @ DSC $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::DSC, @ $($x)*) };
    ($affiliation:expr, @ DSF $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::DSF, @ $($x)*) };
    ($affiliation:expr, @ DDS $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::DDS, @ $($x)*) };
    ($affiliation:expr, @ DDC $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::DDC, @ $($x)*) };
    ($affiliation:expr, @ DDF $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::DDF, @ $($x)*) };
    ($affiliation:expr, @ DFS $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::DFS, @ $($x)*) };
    ($affiliation:expr, @ DFC $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::DFC, @ $($x)*) };
    ($affiliation:expr, @ DFF $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::DFF, @ $($x)*) };
    ($affiliation:expr, @     $($x:ident)*) => { ca!($affiliation, $crate::category::Configuration::UPX, @ $($x)*) };


    ($affiliation:expr, $configuration:expr, @ DEL $($x:ident)*) => { ca!($affiliation, $configuration, $crate::category::Extension::DEL, @ $($x)*) };
    ($affiliation:expr, $configuration:expr, @ PRX $($x:ident)*) => { ca!($affiliation, $configuration, $crate::category::Extension::PRX, @ $($x)*) };
    ($affiliation:expr, $configuration:expr, @ ICP $($x:ident)*) => { ca!($affiliation, $configuration, $crate::category::Extension::ICP, @ $($x)*) };
    ($affiliation:expr, $configuration:expr, @ ATV $($x:ident)*) => { ca!($affiliation, $configuration, $crate::category::Extension::ATV, @ $($x)*) };
    ($affiliation:expr, $configuration:expr, @ GRA $($x:ident)*) => { ca!($affiliation, $configuration, $crate::category::Extension::GRA, @ $($x)*) };
    ($affiliation:expr, $configuration:expr, @ DPL $($x:ident)*) => { ca!($affiliation, $configuration, $crate::category::Extension::DPL, @ $($x)*) };
    ($affiliation:expr, $configuration:expr, @     $($x:ident)*) => { ca!($affiliation, $configuration, $crate::category::Extension::DEL, @ $($x)*) };

    ($affiliation:expr, $configuration:expr, $extension:expr, @ M $($x:ident)*) => { ca!($affiliation, $configuration, $extension, $crate::category::Perspective::M, @ $($x)*) };
    ($affiliation:expr, $configuration:expr, $extension:expr, @ G $($x:ident)*) => { ca!($affiliation, $configuration, $extension, $crate::category::Perspective::G, @ $($x)*) };
    ($affiliation:expr, $configuration:expr, $extension:expr, @ N $($x:ident)*) => { ca!($affiliation, $configuration, $extension, $crate::category::Perspective::N, @ $($x)*) };
    ($affiliation:expr, $configuration:expr, $extension:expr, @ A $($x:ident)*) => { ca!($affiliation, $configuration, $extension, $crate::category::Perspective::A, @ $($x)*) };
    ($affiliation:expr, $configuration:expr, $extension:expr, @   $($x:ident)*) => { ca!($affiliation, $configuration, $extension, $crate::category::Perspective::M, @ $($x)*) };

    ($affiliation:expr, $configuration:expr, $extension:expr, $perspective:expr, @ NRM $($x:ident)*) => { ca!($affiliation, $configuration, $extension, $perspective, $crate::category::Essence::NRM, @) };
    ($affiliation:expr, $configuration:expr, $extension:expr, $perspective:expr, @ RPV $($x:ident)*) => { ca!($affiliation, $configuration, $extension, $perspective, $crate::category::Essence::RPV, @) };
    ($affiliation:expr, $configuration:expr, $extension:expr, $perspective:expr, @                 ) => { ca!($affiliation, $configuration, $extension, $perspective, $crate::category::Essence::NRM, @) };
    ($affiliation:expr, $configuration:expr, $extension:expr, $perspective:expr, @     $($x:ident)*) => { compile_error!("either tokens specified in incorrect order or invalid tokens were specified
help: specify affiliation, configuration, extension, perspective, and essence in that order
help: make sure to separate them with commas
help: any and all segments may be omitted") };

    ($affiliation:expr, $configuration:expr, $extension:expr, $perspective:expr, $essence:expr, @) => {
        $crate::category::Ca {
            affiliation: $affiliation,
            configuration: $configuration,
            extension: $extension,
            perspective: $perspective,
            essence: $essence,
        }
    };

    ($($x:ident),+ ,) => {
        ca!(@ $($x),+)
    };

    ($($x:ident),*) => {
        ca!(@ $($x)*)
    };
}

#[macro_export]
/// Creates a [`Ca`][crate::category::Ca] using more concise syntax that writing it out in full.
/// Also works in `const` contexts.
///
/// # Examples
///
/// ```
/// # use tnil::{ca, ca_pat};
/// # use tnil::category::{Ca, Affiliation, Configuration, Extension, Perspective, Essence};
/// fn affiliation_if_standalone(ca: Ca) -> &'static str {
///     match ca {
///         ca_pat!(CSL) => "CSL",
///         ca_pat!(ASO) => "ASO",
///         ca_pat!(COA) => "COA",
///         ca_pat!(VAR) => "VAR",
///         _ => "not just affiliation",
///     }
/// }
///
/// # fn main() {
/// assert_eq!(affiliation_if_standalone(ca!(ASO)), "ASO");
/// assert_eq!(affiliation_if_standalone(ca!(COA, DPX)), "not just affiliation");
/// assert_eq!(affiliation_if_standalone(ca!(VAR, M)), "VAR");
/// # }
/// ```
macro_rules! ca_pat {
    (@ CSL $($x:ident)*) => { ca_pat!($crate::category::Affiliation::CSL, @ $($x)*) };
    (@ ASO $($x:ident)*) => { ca_pat!($crate::category::Affiliation::ASO, @ $($x)*) };
    (@ COA $($x:ident)*) => { ca_pat!($crate::category::Affiliation::COA, @ $($x)*) };
    (@ VAR $($x:ident)*) => { ca_pat!($crate::category::Affiliation::VAR, @ $($x)*) };
    (@     $($x:ident)*) => { ca_pat!($crate::category::Affiliation::CSL, @ $($x)*) };

    ($affiliation:pat, @ UPX $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::UPX, @ $($x)*) };
    ($affiliation:pat, @ MSS $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::MSS, @ $($x)*) };
    ($affiliation:pat, @ MSC $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::MSC, @ $($x)*) };
    ($affiliation:pat, @ MSF $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::MSF, @ $($x)*) };
    ($affiliation:pat, @ MDS $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::MDS, @ $($x)*) };
    ($affiliation:pat, @ MDC $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::MDC, @ $($x)*) };
    ($affiliation:pat, @ MDF $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::MDF, @ $($x)*) };
    ($affiliation:pat, @ MFS $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::MFS, @ $($x)*) };
    ($affiliation:pat, @ MFC $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::MFC, @ $($x)*) };
    ($affiliation:pat, @ MFF $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::MFF, @ $($x)*) };
    ($affiliation:pat, @ DPX $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::DPX, @ $($x)*) };
    ($affiliation:pat, @ DSS $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::DSS, @ $($x)*) };
    ($affiliation:pat, @ DSC $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::DSC, @ $($x)*) };
    ($affiliation:pat, @ DSF $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::DSF, @ $($x)*) };
    ($affiliation:pat, @ DDS $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::DDS, @ $($x)*) };
    ($affiliation:pat, @ DDC $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::DDC, @ $($x)*) };
    ($affiliation:pat, @ DDF $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::DDF, @ $($x)*) };
    ($affiliation:pat, @ DFS $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::DFS, @ $($x)*) };
    ($affiliation:pat, @ DFC $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::DFC, @ $($x)*) };
    ($affiliation:pat, @ DFF $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::DFF, @ $($x)*) };
    ($affiliation:pat, @     $($x:ident)*) => { ca_pat!($affiliation, $crate::category::Configuration::UPX, @ $($x)*) };


    ($affiliation:pat, $configuration:pat, @ DEL $($x:ident)*) => { ca_pat!($affiliation, $configuration, $crate::category::Extension::DEL, @ $($x)*) };
    ($affiliation:pat, $configuration:pat, @ PRX $($x:ident)*) => { ca_pat!($affiliation, $configuration, $crate::category::Extension::PRX, @ $($x)*) };
    ($affiliation:pat, $configuration:pat, @ ICP $($x:ident)*) => { ca_pat!($affiliation, $configuration, $crate::category::Extension::ICP, @ $($x)*) };
    ($affiliation:pat, $configuration:pat, @ ATV $($x:ident)*) => { ca_pat!($affiliation, $configuration, $crate::category::Extension::ATV, @ $($x)*) };
    ($affiliation:pat, $configuration:pat, @ GRA $($x:ident)*) => { ca_pat!($affiliation, $configuration, $crate::category::Extension::GRA, @ $($x)*) };
    ($affiliation:pat, $configuration:pat, @ DPL $($x:ident)*) => { ca_pat!($affiliation, $configuration, $crate::category::Extension::DPL, @ $($x)*) };
    ($affiliation:pat, $configuration:pat, @     $($x:ident)*) => { ca_pat!($affiliation, $configuration, $crate::category::Extension::DEL, @ $($x)*) };

    ($affiliation:pat, $configuration:pat, $extension:pat, @ M $($x:ident)*) => { ca_pat!($affiliation, $configuration, $extension, $crate::category::Perspective::M, @ $($x)*) };
    ($affiliation:pat, $configuration:pat, $extension:pat, @ G $($x:ident)*) => { ca_pat!($affiliation, $configuration, $extension, $crate::category::Perspective::G, @ $($x)*) };
    ($affiliation:pat, $configuration:pat, $extension:pat, @ N $($x:ident)*) => { ca_pat!($affiliation, $configuration, $extension, $crate::category::Perspective::N, @ $($x)*) };
    ($affiliation:pat, $configuration:pat, $extension:pat, @ A $($x:ident)*) => { ca_pat!($affiliation, $configuration, $extension, $crate::category::Perspective::A, @ $($x)*) };
    ($affiliation:pat, $configuration:pat, $extension:pat, @   $($x:ident)*) => { ca_pat!($affiliation, $configuration, $extension, $crate::category::Perspective::M, @ $($x)*) };

    ($affiliation:pat, $configuration:pat, $extension:pat, $perspective:pat, @ NRM $($x:ident)*) => { ca_pat!($affiliation, $configuration, $extension, $perspective, $crate::category::Essence::NRM, @) };
    ($affiliation:pat, $configuration:pat, $extension:pat, $perspective:pat, @ RPV $($x:ident)*) => { ca_pat!($affiliation, $configuration, $extension, $perspective, $crate::category::Essence::RPV, @) };
    ($affiliation:pat, $configuration:pat, $extension:pat, $perspective:pat, @                 ) => { ca_pat!($affiliation, $configuration, $extension, $perspective, $crate::category::Essence::NRM, @) };
    ($affiliation:pat, $configuration:pat, $extension:pat, $perspective:pat, @     $($x:ident)*) => { compile_error!("either tokens specified in incorrect order or invalid tokens were specified
help: specify affiliation, configuration, extension, perspective, and essence in that order
help: make sure to separate them with commas
help: any and all segments may be omitted") };

    ($affiliation:pat, $configuration:pat, $extension:pat, $perspective:pat, $essence:pat, @) => {
        $crate::category::Ca {
            affiliation: $affiliation,
            configuration: $configuration,
            extension: $extension,
            perspective: $perspective,
            essence: $essence,
        }
    };

    ($($x:ident),+ ,) => {
        ca_pat!(@ $($x),+)
    };

    ($($x:ident),*) => {
        ca_pat!(@ $($x)*)
    };
}
