use std::{fmt, str::FromStr};

use rand::seq::SliceRandom;

use crate::error::{KodeKrakenError, Result};

// TODO: Add more exceptions, and move to a separate file
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum KotlinExceptions {
    ArithmArithmeticException,
    NullPointerException,
    IllegalArgumentException,
    IllegalStateException,
    IndexOutOfBoundsException,
    NoSuchElementException,
    UnsupportedOperationException,
}

impl fmt::Display for KotlinExceptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KotlinExceptions::ArithmArithmeticException => write!(f, "ArithmeticException"),
            KotlinExceptions::NullPointerException => write!(f, "NullPointerException"),
            KotlinExceptions::IllegalArgumentException => write!(f, "IllegalArgumentException"),
            KotlinExceptions::IllegalStateException => write!(f, "IllegalStateException"),
            KotlinExceptions::IndexOutOfBoundsException => write!(f, "IndexOutOfBoundsException"),
            KotlinExceptions::NoSuchElementException => write!(f, "NoSuchElementException"),
            KotlinExceptions::UnsupportedOperationException => {
                write!(f, "UnsupportedOperationException")
            }
        }
    }
}

impl FromStr for KotlinExceptions {
    type Err = KodeKrakenError;

    fn from_str(s: &str) -> Result<Self> {
        let res = match s {
            "ArithmeticException" => KotlinExceptions::ArithmArithmeticException,
            "NullPointerException" => KotlinExceptions::NullPointerException,
            "IllegalArgumentException" => KotlinExceptions::IllegalArgumentException,
            "IllegalStateException" => KotlinExceptions::IllegalStateException,
            "IndexOutOfBoundsException" => KotlinExceptions::IndexOutOfBoundsException,
            "NoSuchElementException" => KotlinExceptions::NoSuchElementException,
            "UnsupportedOperationException" => KotlinExceptions::UnsupportedOperationException,
            _ => return Err(KodeKrakenError::ConversionError),
        };
        Ok(res)
    }
}

impl KotlinExceptions {
    pub fn get_all_exceptions() -> Vec<KotlinExceptions> {
        vec![
            KotlinExceptions::ArithmArithmeticException,
            KotlinExceptions::NullPointerException,
            KotlinExceptions::IllegalArgumentException,
            KotlinExceptions::IllegalStateException,
            KotlinExceptions::IndexOutOfBoundsException,
            KotlinExceptions::NoSuchElementException,
            KotlinExceptions::UnsupportedOperationException,
        ]
    }

    pub fn get_random_exception(&self) -> KotlinExceptions {
        let mut rng = rand::thread_rng();
        let exceptions = KotlinExceptions::get_all_exceptions();
        let mut rnd = self;
        while rnd == self {
            rnd = exceptions.choose(&mut rng).unwrap();
        }

        *rnd
    }
}

/// Holds all characters that are not named in kotlin
const NON_NAMED_TYPES: [&str; 128] = [
    "!",
    "!!",
    "!=",
    "!==",
    "!in",
    "\"",
    "\"\"\"",
    "#!",
    "$",
    "${",
    "%",
    "%=",
    "&&",
    "'",
    "(",
    ")",
    "*",
    "*=",
    "+",
    "++",
    "+=",
    ",",
    "-",
    "--",
    "-=",
    "->",
    ".",
    ".*",
    "..",
    "/",
    "/=",
    ":",
    "::",
    ";",
    "<",
    "<=",
    "=",
    "==",
    "===",
    ">",
    ">=",
    "?:",
    "@",
    "L",
    "[",
    "\\",
    "]",
    "abstract",
    "actual",
    "annotation",
    "as",
    "as?",
    "break",
    "break@",
    "by",
    "catch",
    "class",
    "companion",
    "constructor",
    "continue",
    "continue@",
    "crossinline",
    "data",
    "delegate",
    "do",
    "dynamic",
    "else",
    "enum",
    "expect",
    "external",
    "false",
    "field",
    "file",
    "final",
    "finally",
    "for",
    "fun",
    "get",
    "if",
    "import",
    "in",
    "infix",
    "init",
    "inline",
    "inner",
    "interface",
    "internal",
    "is",
    "lateinit",
    "noinline",
    "null",
    "object",
    "open",
    "operator",
    "out",
    "override",
    "package",
    "param",
    "private",
    "property",
    "protected",
    "public",
    "receiver",
    "return",
    "return@",
    "sealed",
    "set",
    "setparam",
    "super",
    "super@",
    "suspend",
    "tailrec",
    "this",
    "this@",
    "throw",
    "true",
    "try",
    "typealias",
    "u",
    "val",
    "var",
    "vararg",
    "when",
    "where",
    "while",
    "{",
    "||",
    "}",
];

/// An enum for all the types that exist within kotlin
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KotlinTypes {
    /// Custom type
    AnyParent,
    AdditiveExpression,
    AnnotatedLambda,
    Annotation,
    AnonymousFunction,
    AnonymousInitializer,
    AsExpression,
    Assignment,
    BooleanLiteral,
    CallExpression,
    CallSuffix,
    CallableReference,
    CatchBlock,
    CharacterLiteral,
    CheckExpression,
    ClassBody,
    ClassDeclaration,
    ClassModifier,
    ClassParameter,
    CollectionLiteral,
    CompanionObject,
    ComparisonExpression,
    ConjunctionExpression,
    ConstructorDelegationCall,
    ConstructorInvocation,
    ControlStructureBody,
    DelegationSpecifier,
    DirectlyAssignableExpression,
    DisjunctionExpression,
    DoWhileStatement,
    ElvisExpression,
    EnumClassBody,
    EnumEntry,
    EqualityExpression,
    ExplicitDelegation,
    FileAnnotation,
    FinallyBlock,
    ForStatement,
    FunctionBody,
    FunctionDeclaration,
    FunctionModifier,
    FunctionType,
    FunctionTypeParameters,
    Getter,
    Identifier,
    IfExpression,
    ImportAlias,
    ImportHeader,
    IndexingExpression,
    IndexingSuffix,
    InfixExpression,
    InheritanceModifier,
    InterpolatedExpression,
    InterpolatedIdentifier,
    JumpExpression,
    LambdaLiteral,
    LambdaParameters,
    LineStringLiteral,
    LongLiteral,
    MemberModifier,
    Modifiers,
    MultiLineStringLiteral,
    MultiplicativeExpression,
    NavigationExpression,
    NavigationSuffix,
    NullableType,
    ObjectDeclaration,
    ObjectLiteral,
    PackageHeader,
    Parameter,
    ParameterModifier,
    ParameterModifiers,
    ParameterWithOptionalType,
    ParenthesizedExpression,
    ParenthesizedType,
    ParenthesizedUserType,
    PlatformModifier,
    PostfixExpression,
    PrefixExpression,
    PrimaryConstructor,
    PropertyDeclaration,
    PropertyDelegate,
    RangeExpression,
    RangeTest,
    /// Custom type for the removal of an operator
    RemoveOperator,
    SecondaryConstructor,
    Setter,
    ShebangLine,
    SimpleIdentifier,
    SourceFile,
    SpreadExpression,
    Statements,
    SuperExpression,
    ThisExpression,
    TryExpression,
    TypeAlias,
    TypeArguments,
    TypeConstraint,
    TypeConstraints,
    TypeIdentifier,
    TypeModifiers,
    TypeParameter,
    TypeParameterModifiers,
    TypeParameters,
    TypeProjection,
    TypeProjectionModifiers,
    TypeTest,
    UnsignedLiteral,
    UseSiteTarget,
    UserType,
    ValueArgument,
    ValueArguments,
    VariableDeclaration,
    VarianceModifier,
    VisibilityModifier,
    WhenCondition,
    WhenEntry,
    WhenExpression,
    WhenSubject,
    WhileStatement,
    BinLiteral,
    Comment,
    HexLiteral,
    IntegerLiteral,
    Label,
    PropertyModifier,
    RealLiteral,
    ReificationModifier,
    Error,
    NonNamedType(String),
}

impl fmt::Display for KotlinTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KotlinTypes::AdditiveExpression => write!(f, "AdditiveExpression"),
            KotlinTypes::AnnotatedLambda => write!(f, "AnnotatedLambda"),
            KotlinTypes::Annotation => write!(f, "Annotation"),
            KotlinTypes::AnonymousFunction => write!(f, "AnonymousFunction"),
            KotlinTypes::AnonymousInitializer => write!(f, "AnonymousInitializer"),
            KotlinTypes::AsExpression => write!(f, "AsExpression"),
            KotlinTypes::Assignment => write!(f, "Assignment"),
            KotlinTypes::BooleanLiteral => write!(f, "BooleanLiteral"),
            KotlinTypes::CallExpression => write!(f, "CallExpression"),
            KotlinTypes::CallSuffix => write!(f, "CallSuffix"),
            KotlinTypes::CallableReference => write!(f, "CallableReference"),
            KotlinTypes::CatchBlock => write!(f, "CatchBlock"),
            KotlinTypes::CharacterLiteral => write!(f, "CharacterLiteral"),
            KotlinTypes::CheckExpression => write!(f, "CheckExpression"),
            KotlinTypes::ClassBody => write!(f, "ClassBody"),
            KotlinTypes::ClassDeclaration => write!(f, "ClassDeclaration"),
            KotlinTypes::ClassModifier => write!(f, "ClassModifier"),
            KotlinTypes::ClassParameter => write!(f, "ClassParameter"),
            KotlinTypes::CollectionLiteral => write!(f, "CollectionLiteral"),
            KotlinTypes::CompanionObject => write!(f, "CompanionObject"),
            KotlinTypes::ComparisonExpression => write!(f, "ComparisonExpression"),
            KotlinTypes::ConjunctionExpression => write!(f, "ConjunctionExpression"),
            KotlinTypes::ConstructorDelegationCall => write!(f, "ConstructorDelegationCall"),
            KotlinTypes::ConstructorInvocation => write!(f, "ConstructorInvocation"),
            KotlinTypes::ControlStructureBody => write!(f, "ControlStructureBody"),
            KotlinTypes::DelegationSpecifier => write!(f, "DelegationSpecifier"),
            KotlinTypes::DirectlyAssignableExpression => write!(f, "DirectlyAssignableExpression"),
            KotlinTypes::DisjunctionExpression => write!(f, "DisjunctionExpression"),
            KotlinTypes::DoWhileStatement => write!(f, "DoWhileStatement"),
            KotlinTypes::ElvisExpression => write!(f, "ElvisExpression"),
            KotlinTypes::EnumClassBody => write!(f, "EnumClassBody"),
            KotlinTypes::EnumEntry => write!(f, "EnumEntry"),
            KotlinTypes::EqualityExpression => write!(f, "EqualityExpression"),
            KotlinTypes::ExplicitDelegation => write!(f, "ExplicitDelegation"),
            KotlinTypes::FileAnnotation => write!(f, "FileAnnotation"),
            KotlinTypes::FinallyBlock => write!(f, "FinallyBlock"),
            KotlinTypes::ForStatement => write!(f, "ForStatement"),
            KotlinTypes::FunctionBody => write!(f, "FunctionBody"),
            KotlinTypes::FunctionDeclaration => write!(f, "FunctionDeclaration"),
            KotlinTypes::FunctionModifier => write!(f, "FunctionModifier"),
            KotlinTypes::FunctionType => write!(f, "FunctionType"),
            KotlinTypes::FunctionTypeParameters => write!(f, "FunctionTypeParameters"),
            KotlinTypes::Getter => write!(f, "Getter"),
            KotlinTypes::Identifier => write!(f, "Identifier"),
            KotlinTypes::IfExpression => write!(f, "IfExpression"),
            KotlinTypes::ImportAlias => write!(f, "ImportAlias"),
            KotlinTypes::ImportHeader => write!(f, "ImportHeader"),
            KotlinTypes::IndexingExpression => write!(f, "IndexingExpression"),
            KotlinTypes::IndexingSuffix => write!(f, "IndexingSuffix"),
            KotlinTypes::InfixExpression => write!(f, "InfixExpression"),
            KotlinTypes::InheritanceModifier => write!(f, "InheritanceModifier"),
            KotlinTypes::InterpolatedExpression => write!(f, "InterpolatedExpression"),
            KotlinTypes::InterpolatedIdentifier => write!(f, "InterpolatedIdentifier"),
            KotlinTypes::JumpExpression => write!(f, "JumpExpression"),
            KotlinTypes::LambdaLiteral => write!(f, "LambdaLiteral"),
            KotlinTypes::LambdaParameters => write!(f, "LambdaParameters"),
            KotlinTypes::LineStringLiteral => write!(f, "LineStringLiteral"),
            KotlinTypes::LongLiteral => write!(f, "LongLiteral"),
            KotlinTypes::MemberModifier => write!(f, "MemberModifier"),
            KotlinTypes::Modifiers => write!(f, "Modifiers"),
            KotlinTypes::MultiLineStringLiteral => write!(f, "MultiLineStringLiteral"),
            KotlinTypes::MultiplicativeExpression => write!(f, "MultiplicativeExpression"),
            KotlinTypes::NavigationExpression => write!(f, "NavigationExpression"),
            KotlinTypes::NavigationSuffix => write!(f, "NavigationSuffix"),
            KotlinTypes::NullableType => write!(f, "NullableType"),
            KotlinTypes::ObjectDeclaration => write!(f, "ObjectDeclaration"),
            KotlinTypes::ObjectLiteral => write!(f, "ObjectLiteral"),
            KotlinTypes::PackageHeader => write!(f, "PackageHeader"),
            KotlinTypes::Parameter => write!(f, "Parameter"),
            KotlinTypes::ParameterModifier => write!(f, "ParameterModifier"),
            KotlinTypes::ParameterModifiers => write!(f, "ParameterModifiers"),
            KotlinTypes::ParameterWithOptionalType => write!(f, "ParameterWithOptionalType"),
            KotlinTypes::ParenthesizedExpression => write!(f, "ParenthesizedExpression"),
            KotlinTypes::ParenthesizedType => write!(f, "ParenthesizedType"),
            KotlinTypes::ParenthesizedUserType => write!(f, "ParenthesizedUserType"),
            KotlinTypes::PlatformModifier => write!(f, "PlatformModifier"),
            KotlinTypes::PostfixExpression => write!(f, "PostfixExpression"),
            KotlinTypes::PrefixExpression => write!(f, "PrefixExpression"),
            KotlinTypes::PrimaryConstructor => write!(f, "PrimaryConstructor"),
            KotlinTypes::PropertyDeclaration => write!(f, "PropertyDeclaration"),
            KotlinTypes::PropertyDelegate => write!(f, "PropertyDelegate"),
            KotlinTypes::RangeExpression => write!(f, "RangeExpression"),
            KotlinTypes::RemoveOperator => write!(f, ""),
            KotlinTypes::RangeTest => write!(f, "RangeTest"),
            KotlinTypes::SecondaryConstructor => write!(f, "SecondaryConstructor"),
            KotlinTypes::Setter => write!(f, "Setter"),
            KotlinTypes::ShebangLine => write!(f, "ShebangLine"),
            KotlinTypes::SimpleIdentifier => write!(f, "SimpleIdentifier"),
            KotlinTypes::SourceFile => write!(f, "SourceFile"),
            KotlinTypes::SpreadExpression => write!(f, "SpreadExpression"),
            KotlinTypes::Statements => write!(f, "Statements"),
            KotlinTypes::SuperExpression => write!(f, "SuperExpression"),
            KotlinTypes::ThisExpression => write!(f, "ThisExpression"),
            KotlinTypes::TryExpression => write!(f, "TryExpression"),
            KotlinTypes::TypeAlias => write!(f, "TypeAlias"),
            KotlinTypes::TypeArguments => write!(f, "TypeArguments"),
            KotlinTypes::TypeConstraint => write!(f, "TypeConstraint"),
            KotlinTypes::TypeConstraints => write!(f, "TypeConstraints"),
            KotlinTypes::TypeIdentifier => write!(f, "TypeIdentifier"),
            KotlinTypes::TypeModifiers => write!(f, "TypeModifiers"),
            KotlinTypes::TypeParameter => write!(f, "TypeParameter"),
            KotlinTypes::TypeParameterModifiers => write!(f, "TypeParameterModifiers"),
            KotlinTypes::TypeParameters => write!(f, "TypeParameters"),
            KotlinTypes::TypeProjection => write!(f, "TypeProjection"),
            KotlinTypes::TypeProjectionModifiers => write!(f, "TypeProjectionModifiers"),
            KotlinTypes::TypeTest => write!(f, "TypeTest"),
            KotlinTypes::UnsignedLiteral => write!(f, "UnsignedLiteral"),
            KotlinTypes::UseSiteTarget => write!(f, "UseSiteTarget"),
            KotlinTypes::UserType => write!(f, "UserType"),
            KotlinTypes::ValueArgument => write!(f, "ValueArgument"),
            KotlinTypes::ValueArguments => write!(f, "ValueArguments"),
            KotlinTypes::VariableDeclaration => write!(f, "VariableDeclaration"),
            KotlinTypes::VarianceModifier => write!(f, "VarianceModifier"),
            KotlinTypes::VisibilityModifier => write!(f, "VisibilityModifier"),
            KotlinTypes::WhenCondition => write!(f, "WhenCondition"),
            KotlinTypes::WhenEntry => write!(f, "WhenEntry"),
            KotlinTypes::WhenExpression => write!(f, "WhenExpression"),
            KotlinTypes::WhenSubject => write!(f, "WhenSubject"),
            KotlinTypes::WhileStatement => write!(f, "WhileStatement"),
            KotlinTypes::BinLiteral => write!(f, "BinLiteral"),
            KotlinTypes::Comment => write!(f, "Comment"),
            KotlinTypes::HexLiteral => write!(f, "HexLiteral"),
            KotlinTypes::IntegerLiteral => write!(f, "IntegerLiteral"),
            KotlinTypes::Label => write!(f, "Label"),
            KotlinTypes::PropertyModifier => write!(f, "PropertyModifier"),
            KotlinTypes::RealLiteral => write!(f, "RealLiteral"),
            KotlinTypes::ReificationModifier => write!(f, "ReificationModifier"),
            KotlinTypes::NonNamedType(s) => write!(f, "{s}"),
            KotlinTypes::Error => write!(f, "Error"),
            KotlinTypes::AnyParent => write!(f, "AnyParent"),
        }
    }
}

impl KotlinTypes {
    pub fn new(s: &str) -> Result<KotlinTypes> {
        let binding: String = s
            .split('_')
            .map(|p| {
                if !p.is_empty() && !NON_NAMED_TYPES.contains(&s) {
                    let mut v: Vec<char> = p.chars().collect();
                    v[0] = v[0].to_uppercase().next().unwrap();
                    let x: String = v.into_iter().collect();
                    x
                } else {
                    p.to_string()
                }
            })
            .collect();
        let s = binding;
        let res = match s.as_str() {
            "AdditiveExpression" => KotlinTypes::AdditiveExpression,
            "AnnotatedLambda" => KotlinTypes::AnnotatedLambda,
            "Annotation" => KotlinTypes::Annotation,
            "AnonymousFunction" => KotlinTypes::AnonymousFunction,
            "AnonymousInitializer" => KotlinTypes::AnonymousInitializer,
            "AsExpression" => KotlinTypes::AsExpression,
            "Assignment" => KotlinTypes::Assignment,
            "BooleanLiteral" => KotlinTypes::BooleanLiteral,
            "CallExpression" => KotlinTypes::CallExpression,
            "CallSuffix" => KotlinTypes::CallSuffix,
            "CallableReference" => KotlinTypes::CallableReference,
            "CatchBlock" => KotlinTypes::CatchBlock,
            "CharacterLiteral" => KotlinTypes::CharacterLiteral,
            "CheckExpression" => KotlinTypes::CheckExpression,
            "ClassBody" => KotlinTypes::ClassBody,
            "ClassDeclaration" => KotlinTypes::ClassDeclaration,
            "ClassModifier" => KotlinTypes::ClassModifier,
            "ClassParameter" => KotlinTypes::ClassParameter,
            "CollectionLiteral" => KotlinTypes::CollectionLiteral,
            "CompanionObject" => KotlinTypes::CompanionObject,
            "ComparisonExpression" => KotlinTypes::ComparisonExpression,
            "ConjunctionExpression" => KotlinTypes::ConjunctionExpression,
            "ConstructorDelegationCall" => KotlinTypes::ConstructorDelegationCall,
            "ConstructorInvocation" => KotlinTypes::ConstructorInvocation,
            "ControlStructureBody" => KotlinTypes::ControlStructureBody,
            "DelegationSpecifier" => KotlinTypes::DelegationSpecifier,
            "DirectlyAssignableExpression" => KotlinTypes::DirectlyAssignableExpression,
            "DisjunctionExpression" => KotlinTypes::DisjunctionExpression,
            "DoWhileStatement" => KotlinTypes::DoWhileStatement,
            "ElvisExpression" => KotlinTypes::ElvisExpression,
            "EnumClassBody" => KotlinTypes::EnumClassBody,
            "EnumEntry" => KotlinTypes::EnumEntry,
            "EqualityExpression" => KotlinTypes::EqualityExpression,
            "ExplicitDelegation" => KotlinTypes::ExplicitDelegation,
            "FileAnnotation" => KotlinTypes::FileAnnotation,
            "FinallyBlock" => KotlinTypes::FinallyBlock,
            "ForStatement" => KotlinTypes::ForStatement,
            "FunctionBody" => KotlinTypes::FunctionBody,
            "FunctionDeclaration" => KotlinTypes::FunctionDeclaration,
            "FunctionModifier" => KotlinTypes::FunctionModifier,
            "FunctionType" => KotlinTypes::FunctionType,
            "FunctionTypeParameters" => KotlinTypes::FunctionTypeParameters,
            "Getter" => KotlinTypes::Getter,
            "Identifier" => KotlinTypes::Identifier,
            "IfExpression" => KotlinTypes::IfExpression,
            "ImportAlias" => KotlinTypes::ImportAlias,
            "ImportHeader" => KotlinTypes::ImportHeader,
            "IndexingExpression" => KotlinTypes::IndexingExpression,
            "IndexingSuffix" => KotlinTypes::IndexingSuffix,
            "InfixExpression" => KotlinTypes::InfixExpression,
            "InheritanceModifier" => KotlinTypes::InheritanceModifier,
            "InterpolatedExpression" => KotlinTypes::InterpolatedExpression,
            "InterpolatedIdentifier" => KotlinTypes::InterpolatedIdentifier,
            "JumpExpression" => KotlinTypes::JumpExpression,
            "LambdaLiteral" => KotlinTypes::LambdaLiteral,
            "LambdaParameters" => KotlinTypes::LambdaParameters,
            "LineStringLiteral" => KotlinTypes::LineStringLiteral,
            "LongLiteral" => KotlinTypes::LongLiteral,
            "MemberModifier" => KotlinTypes::MemberModifier,
            "Modifiers" => KotlinTypes::Modifiers,
            "MultiLineStringLiteral" => KotlinTypes::MultiLineStringLiteral,
            "MultiplicativeExpression" => KotlinTypes::MultiplicativeExpression,
            "NavigationExpression" => KotlinTypes::NavigationExpression,
            "NavigationSuffix" => KotlinTypes::NavigationSuffix,
            "NullableType" => KotlinTypes::NullableType,
            "ObjectDeclaration" => KotlinTypes::ObjectDeclaration,
            "ObjectLiteral" => KotlinTypes::ObjectLiteral,
            "PackageHeader" => KotlinTypes::PackageHeader,
            "Parameter" => KotlinTypes::Parameter,
            "ParameterModifier" => KotlinTypes::ParameterModifier,
            "ParameterModifiers" => KotlinTypes::ParameterModifiers,
            "ParameterWithOptionalType" => KotlinTypes::ParameterWithOptionalType,
            "ParenthesizedExpression" => KotlinTypes::ParenthesizedExpression,
            "ParenthesizedType" => KotlinTypes::ParenthesizedType,
            "ParenthesizedUserType" => KotlinTypes::ParenthesizedUserType,
            "PlatformModifier" => KotlinTypes::PlatformModifier,
            "PostfixExpression" => KotlinTypes::PostfixExpression,
            "PrefixExpression" => KotlinTypes::PrefixExpression,
            "PrimaryConstructor" => KotlinTypes::PrimaryConstructor,
            "PropertyDeclaration" => KotlinTypes::PropertyDeclaration,
            "PropertyDelegate" => KotlinTypes::PropertyDelegate,
            "RangeExpression" => KotlinTypes::RangeExpression,
            "RangeTest" => KotlinTypes::RangeTest,
            "SecondaryConstructor" => KotlinTypes::SecondaryConstructor,
            "Setter" => KotlinTypes::Setter,
            "ShebangLine" => KotlinTypes::ShebangLine,
            "SimpleIdentifier" => KotlinTypes::SimpleIdentifier,
            "SourceFile" => KotlinTypes::SourceFile,
            "SpreadExpression" => KotlinTypes::SpreadExpression,
            "Statements" => KotlinTypes::Statements,
            "SuperExpression" => KotlinTypes::SuperExpression,
            "ThisExpression" => KotlinTypes::ThisExpression,
            "TryExpression" => KotlinTypes::TryExpression,
            "TypeAlias" => KotlinTypes::TypeAlias,
            "TypeArguments" => KotlinTypes::TypeArguments,
            "TypeConstraint" => KotlinTypes::TypeConstraint,
            "TypeConstraints" => KotlinTypes::TypeConstraints,
            "TypeIdentifier" => KotlinTypes::TypeIdentifier,
            "TypeModifiers" => KotlinTypes::TypeModifiers,
            "TypeParameter" => KotlinTypes::TypeParameter,
            "TypeParameterModifiers" => KotlinTypes::TypeParameterModifiers,
            "TypeParameters" => KotlinTypes::TypeParameters,
            "TypeProjection" => KotlinTypes::TypeProjection,
            "TypeProjectionModifiers" => KotlinTypes::TypeProjectionModifiers,
            "TypeTest" => KotlinTypes::TypeTest,
            "UnsignedLiteral" => KotlinTypes::UnsignedLiteral,
            "UseSiteTarget" => KotlinTypes::UseSiteTarget,
            "UserType" => KotlinTypes::UserType,
            "ValueArgument" => KotlinTypes::ValueArgument,
            "ValueArguments" => KotlinTypes::ValueArguments,
            "VariableDeclaration" => KotlinTypes::VariableDeclaration,
            "VarianceModifier" => KotlinTypes::VarianceModifier,
            "VisibilityModifier" => KotlinTypes::VisibilityModifier,
            "WhenCondition" => KotlinTypes::WhenCondition,
            "WhenEntry" => KotlinTypes::WhenEntry,
            "WhenExpression" => KotlinTypes::WhenExpression,
            "WhenSubject" => KotlinTypes::WhenSubject,
            "WhileStatement" => KotlinTypes::WhileStatement,
            "BinLiteral" => KotlinTypes::BinLiteral,
            "Comment" => KotlinTypes::Comment,
            "HexLiteral" => KotlinTypes::HexLiteral,
            "IntegerLiteral" => KotlinTypes::IntegerLiteral,
            "Label" => KotlinTypes::Label,
            "PropertyModifier" => KotlinTypes::PropertyModifier,
            "RealLiteral" => KotlinTypes::RealLiteral,
            "ReificationModifier" => KotlinTypes::ReificationModifier,
            "ERROR" => KotlinTypes::Error,
            "REMOVE" => KotlinTypes::RemoveOperator,
            unnamed => {
                if !NON_NAMED_TYPES.contains(&unnamed) {
                    return Err(KodeKrakenError::ConversionError);
                }
                KotlinTypes::NonNamedType(unnamed.to_string())
            }
        };
        Ok(res)
    }

    pub fn as_str(&self) -> String {
        let mut second_upper = 0;
        let mut x = format!("{}", *self);
        x.as_bytes().iter().enumerate().for_each(|(i, c)| {
            if (*c as char).is_uppercase() && i != 0 {
                second_upper = i
            }
        });
        x = x.to_lowercase();
        if second_upper != 0 {
            x.insert(second_upper, '_');
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use crate::kotlin_types::NON_NAMED_TYPES;

    use super::KotlinTypes;

    #[test]
    fn should_successfully_convert_kotlin_types() {
        let res = KotlinTypes::new("value_argument").unwrap();
        assert!(res == KotlinTypes::ValueArgument);
    }

    #[test]
    fn should_return_an_err_for_converting_kotlin_types() {
        let res = KotlinTypes::new("Not_valid_Type");
        assert!(res.is_err());
    }

    #[test]
    fn should_successfully_convert_kotlin_types_to_string() {
        let res = KotlinTypes::ValueArgument.as_str();
        assert!(res == "value_argument");
    }

    #[test]
    fn should_successfully_convert_non_named_type() {
        let non_named_type = NON_NAMED_TYPES[0];
        let res = KotlinTypes::new(non_named_type).unwrap();
        assert!(res == KotlinTypes::NonNamedType(non_named_type.to_string()));
    }

    #[test]
    fn should_successfully_convert_non_named_type_to_string() {
        let non_named_type = NON_NAMED_TYPES[0];
        let res = KotlinTypes::NonNamedType(non_named_type.to_string()).as_str();
        assert!(res == *non_named_type);
    }
}
