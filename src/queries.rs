use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::query::{AQuery, QueryType, TypeOfFix};

lazy_static! {
    pub static ref QUERIES: HashMap<&'static str, AQuery> = HashMap::from([
        (
            "BuildTimeToolInBuildInputs",
            (AQuery {
                name: "build time tool in buildInputs".to_string(),
                solution: "move this from buildInputs to nativeBuildInputs".to_string(),
                context: String::new(),
                what: "cmake|makeWrapper|pkg-config|intltool|autoreconfHook".to_string(),
                in_what: "buildInputs".to_string(),
                type_of_query: QueryType::List,
                type_of_fix: TypeOfFix::Move,
            }),
        ),
        (
            "FlagsNotList",
            (AQuery {
                name: "*Flags not a list".to_string(),
                solution: "convert to a list".to_string(),
                context: String::new(),
                what: String::new(),
                in_what: "Flags".to_string(),
                type_of_query: QueryType::BindingAStringInsteadOfList,
                type_of_fix: TypeOfFix::ConvertToList,
            }),
        ),
        (
            "ArgsToOptionalIsList",
            (AQuery {
                name: "Arg to lib.optional is a list".to_string(),
                solution: "change lib.optional to lib.optionals".to_string(),
                context: String::new(),
                what: String::new(),
                in_what: String::new(),
                type_of_query: QueryType::ArgToOptionalAList,
                type_of_fix: TypeOfFix::Change,
            }),
        ),
        (
            "UnnormalizedPythonPname",
            (AQuery {
                name: "unnormalized python pname".to_string(),
                solution: "normalize this according to PEP503, for example, lowercase and use `-` instead of `.` and `_`".to_string(),
                context: "buildPythonPackage".to_string(),
                what: "[A-Z._]".to_string(),
                in_what: "pname".to_string(),
                type_of_query: QueryType::AttrValueInContext,
                type_of_fix: TypeOfFix::Change,
            })
        ),
    ]);

    pub static ref UNFINISHED_QUERIES: HashMap<&'static str, AQuery> = HashMap::from([
        (
            "RedundantPackageFromStdenv",
            (AQuery {
                name: "redundant package from stdenv in nativeBuildInputs".to_string(),
                solution: "remove this from nativeBuildInputs".to_string(),
                context: String::new(),
                what: r"coreutils|findutils|diffutils|gnugrep|gawk|gnutar|gzip|bzip2\.bin|gnumake|bash|patch|xz\.bin|file".to_string(),
                in_what: "nativeBuildInputs".to_string(),
                type_of_query: QueryType::List,
                type_of_fix: TypeOfFix::Remove,
            })
        ),
        (
            "StartsWithDefiniteOrIndefiniteArticleInDescription",
            (AQuery {
                name: "starts with definite or indefinite article in description".to_string(),
                solution: "remove a definite/indefinite article from meta.description".to_string(),
                context: String::new(),
                what: r"^(A|The) ".to_string(),
                in_what: "description".to_string(),
                type_of_query: QueryType::String,
                type_of_fix: TypeOfFix::Change,
            })
        ),
        (
            "NoCapitalizationInDescription",
            (AQuery {
                name: "no capitalization in description".to_string(),
                solution: "be capitalized".to_string(),
                context: String::new(),
                what: r"^[a-z]".to_string(),
                in_what: "description".to_string(),
                type_of_query: QueryType::String,
                type_of_fix: TypeOfFix::Change,
            })
        ),
        (
            "EndsWithPeriodInDescription",
            (AQuery {
                name: "ends with period in description".to_string(),
                solution: "remove a period from meta.description".to_string(),
                context: String::new(),
                what: r"\\.$".to_string(),
                in_what: "description".to_string(),
                type_of_query: QueryType::String,
                type_of_fix: TypeOfFix::Change,
            })
        ),
        (
            "RecKeywordWithMkDerivation",
            (AQuery {
                name: "rec keyword with mkDerivation".to_string(),
                solution: "consider to use finalAttrs pattern".to_string(),
                context: String::new(),
                what: String::new(),
                in_what: "mkDerivation".to_string(),
                type_of_query: QueryType::RecursiveAttrSet,
                type_of_fix: TypeOfFix::Change,
            })
        ),
        (
            "RefsTagsWithRev",
            (AQuery {
                name: "refs/tags with rev".to_string(),
                solution: "replace rev with tag and remove the prefix, refs/tags/".to_string(),
                context: "fetchFromGitHub".to_string(),
                what: "^refs/tags/".to_string(),
                in_what: "rev".to_string(),
                type_of_query: QueryType::AttrValueInContext,
                type_of_fix: TypeOfFix::Change,
            })
        ),
        (
            "SRIHash",
            (AQuery {
                name: "SRI hash".to_string(),
                solution: "replace this with hash".to_string(),
                context: String::new(),
                what: "sha256".to_string(),
                in_what: "fetchFromGitHub|fetchPypi".to_string(),
                type_of_query: QueryType::AttrNameInFunction,
                type_of_fix: TypeOfFix::Change,
            })
        ),
        (
            "BindingWithExpression",
            (AQuery {
                name: "binding with expression".to_string(),
                solution: "remove with expression".to_string(),
                context: String::new(),
                what: "lib".to_string(),
                in_what: "meta".to_string(),
                type_of_query: QueryType::BindingWithExpression,
                type_of_fix: TypeOfFix::Change,
            })
        ),
        (
            "DeprecatedFormatAttributeUsage",
            (AQuery {
                name: "deprecated format attribute usage".to_string(),
                solution: "set `pyproject = true` instead".to_string(),
                context: String::new(),
                what: r"setuptools|pyproject".to_string(),
                in_what: "format".to_string(),
                type_of_query: QueryType::String,
                type_of_fix: TypeOfFix::Change,
            })
        ),
        (
            "DeprecatedFormatOtherAttributeUsage",
            (AQuery {
                name: "deprecated format attribute usage".to_string(),
                solution: "set `pyproject = false` instead".to_string(),
                context: String::new(),
                what: r"other".to_string(),
                in_what: "format".to_string(),
                type_of_query: QueryType::String,
                type_of_fix: TypeOfFix::Change,
            })
        ),
        (
            "UnnecessaryWheel",
            (AQuery {
                name: "unnecessary wheel in build-system".to_string(),
                solution: "remove this from build-system".to_string(),
                context: String::new(),
                what: r"wheel".to_string(),
                in_what: "build-system".to_string(),
                type_of_query: QueryType::List,
                type_of_fix: TypeOfFix::Remove,
            })
        ),
        (
            "PythonPackageInNativeBuildInputs",
            (AQuery {
                name: "python package in nativeBuildInputs".to_string(),
                solution: "move this from nativeBuildInputs to build-system".to_string(),
                context: String::new(),
                what: r"setuptools|setuptools-scm|hatchling|flit-core|poetry-core|pdm-backend|wheel|maturinBuildHook|".to_string(),
                in_what: "nativeBuildInputs".to_string(),
                type_of_query: QueryType::List,
                type_of_fix: TypeOfFix::Move,
            })
        ),
        (
            "RedundantPackageInNativeBuildInputs",
            (AQuery {
                name: "redundant package in nativeBuildInputs".to_string(),
                solution: "remove this from nativeBuildInputs".to_string(),
                context: String::new(),
                what: r"pythonRelaxDepsHook".to_string(),
                in_what: "nativeBuildInputs".to_string(),
                type_of_query: QueryType::List,
                type_of_fix: TypeOfFix::Move,
            })
        ),
        (
            "VersionedPackageInDependencies",
            (AQuery {
                name: "versioned package in dependencies".to_string(),
                solution: "change `package_X_Y` to `package`".to_string(),
                context: String::new(),
                what: r"[a-z0-9-]+_[0-9]+".to_string(),
                in_what: "dependencies".to_string(),
                type_of_query: QueryType::List,
                type_of_fix: TypeOfFix::Change,
            })
        ),
        (
            "RemovePytestCov",
            (AQuery {
                name: "pytest-cov in nativeCheckInputs".to_string(),
                solution: "remove this from nativeCheckInputs or change to pytest-cov-stub".to_string(),
                context: String::new(),
                what: r"pytest-cov".to_string(),
                in_what: "nativeCheckInputs".to_string(),
                type_of_query: QueryType::List,
                type_of_fix: TypeOfFix::Remove,
            })
        ),
        (
            "PytestBenchmarkInNativeCheckInputs",
            (AQuery {
                name: "pytest-benchmark in nativeCheckInputs".to_string(),
                solution: "remove this from nativeCheckInputs or pass `--benchmark-disable` to pytestFlagsArray".to_string(),
                context: String::new(),
                what: r"pytest-benchmark".to_string(),
                in_what: "nativeCheckInputs".to_string(),
                type_of_query: QueryType::List,
                type_of_fix: TypeOfFix::Remove,
            })
        ),
        (
            "NonFunctionalTestingToolInNativeCheckInputs",
            (AQuery {
                name: "non functional testing tool in nativeCheckInputs".to_string(),
                solution: "remove this from nativeCheckInputs".to_string(),
                context: String::new(),
                what: r"pytest-runner|flake8|black|isort|coverage|ruff".to_string(),
                in_what: "nativeCheckInputs".to_string(),
                type_of_query: QueryType::List,
                type_of_fix: TypeOfFix::Remove,
            })
        ),
        (
            "DeprecatedTestingToolInNativeCheckInputs",
            (AQuery {
                name: "deprecated testing tool in nativeCheckInputs".to_string(),
                solution: "remove this from nativeCheckInputs".to_string(),
                context: String::new(),
                what: r"nose".to_string(),
                in_what: "nativeCheckInputs".to_string(),
                type_of_query: QueryType::List,
                type_of_fix: TypeOfFix::Remove,
            })
        ),
        (
            "BarePytestInNativeCheckInputs",
            (AQuery {
                name: "bare pytest in nativeCheckInputs".to_string(),
                solution: "change pytest to pytestCheckHook".to_string(),
                context: String::new(),
                what: r"pytest".to_string(),
                in_what: "nativeCheckInputs".to_string(),
                type_of_query: QueryType::List,
                type_of_fix: TypeOfFix::Change,
            })
        ),
    ]);
}

pub fn add_default_queries(queries: &mut Vec<AQuery>) {
    let mut default_queries = QUERIES.values().cloned().collect();

    queries.append(&mut default_queries);
}

pub fn add_unfinished_queries(queries: &mut Vec<AQuery>) {
    let mut unfinished_queries = UNFINISHED_QUERIES.values().cloned().collect();

    queries.append(&mut unfinished_queries);
}
