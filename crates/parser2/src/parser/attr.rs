use std::convert::Infallible;
use unwrap_infallible::UnwrapInfallible;

use super::{
    define_scope, parse_list, token_stream::TokenStream, Checkpoint, ErrProof, Parser, Recovery,
};

use crate::SyntaxKind;

pub(super) fn parse_attr_list<S: TokenStream>(
    parser: &mut Parser<S>,
) -> Result<Option<Checkpoint>, Recovery<ErrProof>> {
    if let Some(SyntaxKind::DocComment) | Some(SyntaxKind::Pound) = parser.current_kind() {
        parser.parse_cp(AttrListScope::default(), None).map(Some)
    } else {
        Ok(None)
    }
}

define_scope! {
    pub(crate) AttrListScope,
    AttrList,
    Override(
        Newline
    )
}
impl super::Parse for AttrListScope {
    type Error = Recovery<ErrProof>;

    fn parse<S: TokenStream>(&mut self, parser: &mut Parser<S>) -> Result<(), Self::Error> {
        loop {
            parser.set_newline_as_trivia(true);
            match parser.current_kind() {
                Some(SyntaxKind::Pound) => {
                    parser.parse(AttrScope::default())?;
                }
                Some(SyntaxKind::DocComment) => parser
                    .parse(DocCommentAttrScope::default())
                    .unwrap_infallible(),
                _ => break,
            };
            parser.set_newline_as_trivia(false);
            if parser.find(SyntaxKind::Newline, None)? {
                parser.bump();
            }
        }
        Ok(())
    }
}

define_scope! {
    AttrScope,
    Attr,
    Inheritance
}
impl super::Parse for AttrScope {
    type Error = Recovery<ErrProof>;

    fn parse<S: TokenStream>(&mut self, parser: &mut Parser<S>) -> Result<(), Self::Error> {
        parser.set_newline_as_trivia(false);
        parser.bump_expected(SyntaxKind::Pound);

        parser.set_scope_recovery_stack(&[SyntaxKind::LParen]);
        if parser.find(
            SyntaxKind::Ident,
            Some("expected an attribute name after `#`"),
        )? {
            parser.bump()
        }

        if parser.current_kind() == Some(SyntaxKind::LParen) {
            parser.pop_recovery_stack();
            parser.parse(AttrArgListScope::default())
        } else {
            Ok(())
        }
    }
}

define_scope! {
    AttrArgListScope,
    AttrArgList,
    Override(Comma, RParen)
}
impl super::Parse for AttrArgListScope {
    type Error = Recovery<ErrProof>;

    fn parse<S: TokenStream>(&mut self, parser: &mut Parser<S>) -> Result<(), Self::Error> {
        parse_list(
            parser,
            false,
            (SyntaxKind::LParen, SyntaxKind::RParen),
            |parser| parser.parse(AttrArgScope::default()),
        )
    }
}

define_scope! {
    AttrArgScope,
    AttrArg,
    Inheritance
}
impl super::Parse for AttrArgScope {
    type Error = Recovery<ErrProof>;

    fn parse<S: TokenStream>(&mut self, parser: &mut Parser<S>) -> Result<(), Self::Error> {
        let msg = Some("expected attribute argument in the form `key: value`");

        parser.set_scope_recovery_stack(&[SyntaxKind::Ident, SyntaxKind::Colon]);
        if parser.find_and_pop(SyntaxKind::Ident, msg)? {
            parser.bump();
        }
        if parser.find_and_pop(SyntaxKind::Colon, msg)? {
            parser.bump();
        }
        if parser.find(SyntaxKind::Ident, msg)? {
            parser.bump();
        }
        Ok(())
    }
}

define_scope! {
    DocCommentAttrScope,
    DocCommentAttr,
    Inheritance
}
impl super::Parse for DocCommentAttrScope {
    type Error = Infallible;

    fn parse<S: TokenStream>(&mut self, parser: &mut Parser<S>) -> Result<(), Self::Error> {
        parser.bump_expected(SyntaxKind::DocComment);
        parser.bump_if(SyntaxKind::Newline);
        Ok(())
    }
}
