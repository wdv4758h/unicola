========================================
Unicola - Play with Unicode
========================================


.. contents:: Table of Contents


About
========================================



FAQ
========================================



Features
========================================

* find East Asian Width


Try it!
========================================

Pre-Built Examples
------------------------------

east_asian_width
++++++++++++++++++++

Build:

.. code-block:: sh

    $ cargo build --release --example east_asian_width

Run:

.. code-block:: sh

    $ ./target/release/examples/east_asian_width "這是測試"
    這 => W
    是 => W
    測 => W
    試 => W
    $ ./target/release/examples/east_asian_width "(￣﹁￣)"
    ( => Na
    ￣ => F
    ﹁ => W
    ￣ => F
    ) => Na
    $ ./target/release/examples/east_asian_width "Σ(￣□￣；"
    Σ => A
    ( => Na
    ￣ => F
    □ => A
    ￣ => F
    ； => F


Verify:

.. code-block:: sh

    $ python -c "import unicodedata; print(tuple(map(unicodedata.east_asian_width, '這是測試')))"
    ('W', 'W', 'W', 'W')
    $ python -c "import unicodedata; print(tuple(map(unicodedata.east_asian_width, '(￣﹁￣)')))"
    ('Na', 'F', 'W', 'F', 'Na')
    $ python -c "import unicodedata; print(tuple(map(unicodedata.east_asian_width, 'Σ(￣□￣；')))"
    ('A', 'Na', 'F', 'A', 'F', 'F')


BYOB (Build Your Own Binary)
------------------------------

1. Create a new ``Cargo`` project, e.g. ``$ cargo new --vcs git --bin myprog && cd myprog``
2. Add ``unicola`` to your ``Cargo.toml``

.. code-block:: toml

    [dependencies]
    unicola = "0.0.0"

3. Use ``unicola`` in your ``src/main.rs``

.. code-block:: rust

    extern crate unicola;

    fn main() {
        println!("The result is {} !", unicola::east_asian_width('測'));
    }

4. Build your program: ``$ cargo build --release``
5. Run your program: ``$ ./target/release/myprog``



Notice
========================================



License
========================================



Special Thanks
========================================
