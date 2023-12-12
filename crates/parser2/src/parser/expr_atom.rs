use std::convert::Infallible;

use rowan::Checkpoint;

use crate::{
    parser::{lit, path},
    SyntaxKind,
};

use super::{
    define_scope,
    expr::{parse_expr, parse_expr_no_struct},
    item::ItemScope,
    parse_list, parse_pat,
    stmt::parse_stmt,
    token_stream::TokenStream,
    ErrProof, Parser, Recovery,
};

pub(super) fn parse_expr_atom<S: TokenStream>(
    parser: &mut Parser<S>,
    allow_record_init: bool,
) -> Result<Checkpoint, Recovery<ErrProof>> {
    use SyntaxKind::*;
    match parser.current_kind() {
        Some(IfKw) => parser.parse_cp(IfExprScope::default(), None),
        Some(MatchKw) => parser.parse_cp(MatchExprScope::default(), None),
        Some(LBrace) => parser.parse_cp(BlockExprScope::default(), None),
        Some(LParen) => parser.parse_cp(ParenScope::default(), None),
        Some(LBracket) => parser.parse_cp(ArrayScope::default(), None),
        Some(kind) if lit::is_lit(kind) => parser
            .parse_cp(LitExprScope::default(), None)
            .map_err(|e| e.into()),
        Some(kind) if path::is_path_segment(kind) => {
            parser.parse_cp(PathExprScope::new(allow_record_init), None)
        }
        _ => parser
            .error_and_recover("expected expression") // xxx
            .map(|()| parser.checkpoint()),
    }
}

define_scope! {
    pub(crate) BlockExprScope,
    BlockExpr,
    Override(
        RBrace,
        Newline,
        LetKw,
        ForKw,
        WhileKw,
        ContinueKw,
        BreakKw,
        ReturnKw
    )
}
impl super::Parse for BlockExprScope {
    type Error = Recovery<ErrProof>;

    fn parse<S: TokenStream>(&mut self, parser: &mut Parser<S>) -> Result<(), Self::Error> {
        parser.bump_expected(SyntaxKind::LBrace);

        loop {
            parser.set_newline_as_trivia(true);
            if parser.current_kind() == Some(SyntaxKind::RBrace) || parser.current_kind().is_none()
            {
                break;
            }

            if parser
                .current_kind()
                .map(SyntaxKind::is_item_head)
                .unwrap_or_default()
            {
                parser.parse(ItemScope::default())?;
                continue;
            }

            parse_stmt(parser)?;

            parser.set_newline_as_trivia(false);
            parser.expect(&[SyntaxKind::Newline, SyntaxKind::RBrace], None)?;
            parser.bump_if(SyntaxKind::Newline);
        }

        if parser.find(SyntaxKind::RBrace, Some("missing closing `}` for block"))? {
            parser.bump();
        }
        Ok(())
    }
}

define_scope! { IfExprScope, IfExpr, Inheritance }
impl super::Parse for IfExprScope {
    type Error = Recovery<ErrProof>;

    fn parse<S: TokenStream>(&mut self, parser: &mut Parser<S>) -> Result<(), Self::Error> {
        parser.bump_expected(SyntaxKind::IfKw);

        parser.set_scope_recovery_stack(&[SyntaxKind::LBrace, SyntaxKind::ElseKw]);
        parse_expr_no_struct(parser)?;

        if parser.find_and_pop(SyntaxKind::LBrace, Some("missing `if` body"))? {
            parser.parse(BlockExprScope::default())?;
        }

        if parser.current_kind() == Some(SyntaxKind::ElseKw) {
            parser.bump();

            parser.expect(&[SyntaxKind::LBrace, SyntaxKind::IfKw], None)?;
            parse_expr(parser)?;
        }
        Ok(())
    }
}

define_scope! { MatchExprScope, MatchExpr, Inheritance }
impl super::Parse for MatchExprScope {
    type Error = Recovery<ErrProof>;

    fn parse<S: TokenStream>(&mut self, parser: &mut Parser<S>) -> Result<(), Self::Error> {
        parser.bump_expected(SyntaxKind::MatchKw);

        parse_expr_no_struct(parser)?;
        if parser.find(SyntaxKind::LBrace, None)? {
            dbg!(parser.current_kind());
            parser.parse(MatchArmListScope::default())?;
        }
        Ok(())
    }
}

define_scope! { MatchArmListScope, MatchArmList, Override(SyntaxKind::Newline, SyntaxKind::RBrace) }
impl super::Parse for MatchArmListScope {
    type Error = Recovery<ErrProof>;

    fn parse<S: TokenStream>(&mut self, parser: &mut Parser<S>) -> Result<(), Self::Error> {
        parser.bump_expected(SyntaxKind::LBrace);

        loop {
            parser.set_newline_as_trivia(true);
            if parser.current_kind() == Some(SyntaxKind::RBrace) {
                break;
            }

            parser.parse(MatchArmScope::default())?;
            parser.set_newline_as_trivia(false);

            parser.expect(&[SyntaxKind::Newline, SyntaxKind::RBrace], None)?;
            if !parser.bump_if(SyntaxKind::Newline) {
                break;
            }
        }
        parser.bump_expected(SyntaxKind::RBrace);
        Ok(())
    }
}

define_scope! { MatchArmScope, MatchArm, Inheritance }
impl super::Parse for MatchArmScope {
    type Error = Recovery<ErrProof>;

    fn parse<S: TokenStream>(&mut self, parser: &mut Parser<S>) -> Result<(), Self::Error> {
        parser.set_newline_as_trivia(false);

        parser.set_scope_recovery_stack(&[SyntaxKind::FatArrow]);
        parse_pat(parser)?;

        if parser.find_and_pop(SyntaxKind::FatArrow, None)? {
            parser.bump();
        }
        parse_expr(parser)
    }
}

define_scope! { pub(crate) LitExprScope, LitExpr, Inheritance }
impl super::Parse for LitExprScope {
    type Error = Infallible;

    /// Caller is expected to verify that the next token is a literal.
    fn parse<S: TokenStream>(&mut self, parser: &mut Parser<S>) -> Result<(), Self::Error> {
        parser.parse(lit::LitScope::default())
    }
}

define_scope! { PathExprScope{ allow_record_init: bool }, PathExpr, Inheritance }
impl super::Parse for PathExprScope {
    type Error = Recovery<ErrProof>;

    fn parse<S: TokenStream>(&mut self, parser: &mut Parser<S>) -> Result<(), Self::Error> {
        // xxx "expected an expression"?
        parser.or_recover(|p| p.parse(path::PathScope::default()))?;

        if parser.current_kind() == Some(SyntaxKind::LBrace) && self.allow_record_init {
            self.set_kind(SyntaxKind::RecordInitExpr);
            parser.parse(RecordFieldListScope::default())?;
        }
        Ok(())
    }
}

define_scope! { RecordFieldListScope, RecordFieldList, Override(RBrace, Comma) }
impl super::Parse for RecordFieldListScope {
    type Error = Recovery<ErrProof>;

    fn parse<S: TokenStream>(&mut self, parser: &mut Parser<S>) -> Result<(), Self::Error> {
        parse_list(
            parser,
            true,
            (SyntaxKind::LBrace, SyntaxKind::RBrace),
            |parser| parser.parse(RecordFieldScope::default()),
        )
    }
}

define_scope! { RecordFieldScope, RecordField, Inheritance }
impl super::Parse for RecordFieldScope {
    type Error = Recovery<ErrProof>;

    fn parse<S: TokenStream>(&mut self, parser: &mut Parser<S>) -> Result<(), Self::Error> {
        parser.set_newline_as_trivia(false);
        parser.bump_if(SyntaxKind::Ident);

        if parser.bump_if(SyntaxKind::Colon) {
            parse_expr(parser)?;
        }
        Ok(())
    }
}

define_scope! { ParenScope, ParenExpr, Override(RParen, Comma) }
impl super::Parse for ParenScope {
    type Error = Recovery<ErrProof>;

    fn parse<S: TokenStream>(&mut self, parser: &mut Parser<S>) -> Result<(), Self::Error> {
        parser.bump_expected(SyntaxKind::LParen);

        if parser.bump_if(SyntaxKind::RParen) {
            self.set_kind(SyntaxKind::TupleExpr);
            return Ok(());
        }

        loop {
            if parser.bump_if(SyntaxKind::RParen) {
                return Ok(());
            }
            parse_expr(parser)?;
            parser.expect(&[SyntaxKind::RParen, SyntaxKind::Comma], None)?;

            if parser.bump_if(SyntaxKind::Comma) {
                self.set_kind(SyntaxKind::TupleExpr);
                continue;
            }
            break;
        }
        parser.bump_expected(SyntaxKind::RParen);
        Ok(())
    }
}

define_scope! {
    ArrayScope,
    ArrayExpr,
    Override(RBracket, Comma, SemiColon)
}
impl super::Parse for ArrayScope {
    type Error = Recovery<ErrProof>;

    fn parse<S: TokenStream>(&mut self, parser: &mut Parser<S>) -> Result<(), Self::Error> {
        parser.bump_expected(SyntaxKind::LBracket);

        if parser.bump_if(SyntaxKind::RBracket) {
            return Ok(());
        }

        parse_expr(parser)?;
        parser.expect(
            &[
                SyntaxKind::SemiColon,
                SyntaxKind::Comma,
                SyntaxKind::RBracket,
            ],
            None,
        )?;

        if parser.bump_if(SyntaxKind::SemiColon) {
            self.set_kind(SyntaxKind::ArrayRepExpr);
            parse_expr(parser)?;
        } else {
            while parser.bump_if(SyntaxKind::Comma) {
                if parser.bump_if(SyntaxKind::RBracket) {
                    return Ok(());
                }

                parse_expr(parser)?;
                parser.expect(&[SyntaxKind::Comma, SyntaxKind::RBracket], None)?;
            }
        }

        if parser.find(
            SyntaxKind::RBracket,
            Some("missing closing `]` in array definition"),
        )? {
            parser.bump();
        }
        Ok(())
    }
}
