from std::macros::inline macro import *

@macro assignToIdent
    %1.i := %2.i
@sub
    %1.i := %2.i + 0
@end

@macro assignToValue
    %1.i := %2.n
@sub
    @if %2.n == 0
        LOOP %1.i DO
            %1.i := %1.i - 1
        END
    @else
        %1.i := 0
        %1.i := %1.i + %2.n
    @end
@end

@macro assignToIdentOpIdent
    %1.i := %2.i %3.o %4.i
@sub
    %1.i := %2.i
    LOOP %3.i DO
        %1.i++
    END
@end

@macro assignToIdentMulValue
    %1.i := %2.i * %3.p
@sub
    # implicitly convert to variable
    $4.i := %3.p

    %1.i := 0
    LOOP %3.i DO
        %1.i += %2.i
    END
@end