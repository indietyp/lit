from std::macros::ifelse macro import *

@macro whileComp
    WHILE %1.p %2.c %3.p %4.b
@sub
    $5.i := 1

    WHILE $5 := 1 DO
        IF %1.p %2.c %3.p THEN
            $5.i := 0
        ELSE
            %4.b
        END
    END
@end