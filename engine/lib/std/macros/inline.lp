@macro inlineIncrement
    %1.i++
@sub
    %1.i := %1.i + 1
@end

@macro inlineDecr
    %1.i--
@sub
    %1.i := %1.i - 1
@end

@macro inlineOp
    %1.i %4.o= %2.p
@sub
    @if %2.p is value
        $3.i := $3.i + %2.p
    @else
        $3.i := %1.i + 0
    @end

    LOOP $3.i DO
        %1.i := %1.i %4.i 1
    END
@end