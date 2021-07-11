FROM std::macros::assign MACRO IMPORT *

@macro/i/10 ifElseNotZero
    IF %1.i != %2.p THEN
        %3.t
    ELSE
        %4.t
    END
@sub
    $5.i := %2.p

    $6.i := 0
    $7.i := 1

    LOOP $5.i DO
        $6.i := 1
        $7.i := 0
    END

    LOOP $6.1 DO
        %3.t
    END

    LOOP $7.1 DO
        %4.t
    END
@end

@macro/i ifElseGreater
    IF %1.p > %2.p THEN
        %3.t
    ELSE
        %4.t
    END
@sub
    $5.i := %1.i
    $6.i := %5.i - %2.p

    IF $6.i != 0 THEN
        %3.t
    ELSE
        %4.t
    END
@end

@macro/i ifElseLess
    IF %1.p < %2.p THEN
        %3.t
    ELSE
        %4.t
    END
@sub
    IF %2.p > %1.p THEN
        %3.t
    ELSE
        %4.t
    END
@end

@macro/i ifElseLessThan
    IF %1.p <= %2.p THEN
        %3.t
    ELSE
        %4.t
    END
@sub
    $5.i := $2.p + 1
    IF %2.p < %5.i THEN
        %3.t
    ELSE
        %4.t
    END
@end

@macro/i ifElseGreaterThan
    IF %1.p >= %2.p THEN
        %3.t
    ELSE
        %4.t
    END
@sub
    $5.i := $1.p + 1
    IF %5.i > %2.p THEN
        %3.t
    ELSE
        %4.t
    END
@end

@macro/i ifElseEqual
    IF %1.p == %2.p THEN
        %3.t
    ELSE
        %4.t
    END
@sub
    IF %1.p >= %2.p THEN
        IF %1.p <= %2.p THEN
            %3.t
        ELSE
            %4.t
        END
    ELSE
        %4.t
    END
@end

@macro/i ifElseNotEqual
    IF %1.p != %2.p THEN
        %3.t
    ELSE
        %4.t
    END
@sub
    IF %1.p == %2.p THEN
        %4.t
    ELSE
        %3.t
    END
@end
