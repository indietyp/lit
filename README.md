# LIT - LOOP Interpreter and Text-Editor

The title is a bit of lie, we also support WHILE c:

## Engine

The LOOP compiler and interpreter is written in Rust and compiled to WASM,
this means the engine is fast, very fast.

Here are some statistics:
    - Parsing and Expansion is done in less than 1ms
    - Throughput: 140k LOOP instructions per seconds (run on a MacBook Pro 2019 13")

This compiler is written for educational purposes first, which means we're losing some speed, but the engine is able to step through the program using a `.step()` method.
You can also just straight up evaluate the whole program using a simple `while`:

```rust
while engine.is_running() {
    engine.step()
}
```

### How does the engine work?

The Engine runs in three different steps:
1) Parsing
2) Expansion
    1) Macro Expansion
    2) Flattening
    3) Modification
3) Construct Runtime

### Macro Expansion

Macros are a very handy thing, they allow us to construct more complex problems which then are expanded into their respective LOOP/WHILE equivalents.
Here is an example of how the macro expansion works:

```
IF x <= y THEN
    z := 1
END

<expands>

IF y >= x THEN
    z := 1
END

<expands>

_0 := y + 1
IF _0 > x THEN
    z := 1
END

<expands>

_0 := y + 1
_1 := _0 - x
_2 := 0
_3 := 1

LOOP _1 DO
    _2 := 1
    _3 := 0
END

LOOP _2 DO
    z := 1
END

# ELSE LOOP is not included, because no ELSE clause provided

<expands>

_0 := y + 1
_1 := _0 - x

LOOP _2 DO
    _2 := _2 - 1
END
_2 := _2 + 0

LOOP _3 DO
    _3 := _3 - 1
END
_3 := _3 + 1

LOOP _1 DO
    _2 := 1
    _3 := 0
END

LOOP _2 DO
    z := 1
END


<expands>

_0 := y + 1
_1 := _0 - x

LOOP _2 DO
    _2 := _2 - 1
END
_2 := _2 + 0

LOOP _3 DO
    _3 := _3 - 1
END
_3 := _3 + 1

LOOP _1 DO
    LOOP _2 DO
        _2 := _2 - 1
    END
    _2 := _2 + 1

    LOOP _3 DO
        _3 := _3 - 1
    END
    _3 := _3 + 0
END

LOOP _2 DO
    z := 1
END

<expands>

_0 := y + 1
_1 := _0 - x

LOOP _2 DO
    _2 := _2 - 1
END
_2 := _2 + 0

LOOP _3 DO
    _3 := _3 - 1
END
_3 := _3 + 1

LOOP _1 DO
    LOOP _2 DO
        _2 := _2 - 1
    END
    _2 := _2 + 1

    LOOP _3 DO
        _3 := _3 - 1
    END
    _3 := _3 + 0
END

LOOP _2 DO
    LOOP z DO
        z := z - 1
    END
    z := x + 1
END
```
