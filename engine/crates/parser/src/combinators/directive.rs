use lexer::Directive;
use lexer::Kind;

macro_rules! create_directive {
    ($name:tt, $( $pattern:pat )|+) => {
        simple_combinator!(dir, $name, $($pattern)|*);
    };
}

create_directive!(macro, Kind::Directive(Directive::Macro(_)));
create_directive!(sub, Kind::Directive(Directive::Sub));
create_directive!(end, Kind::Directive(Directive::End));
create_directive!(if, Kind::Directive(Directive::If));
create_directive!(else, Kind::Directive(Directive::Else));
create_directive!(sep, Kind::Directive(Directive::Sep));
create_directive!(placeholder, Kind::Directive(Directive::Placeholder(_)));
