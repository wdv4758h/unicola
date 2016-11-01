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
* Emoji Alpha Codes to Emoji transform


Try it!
========================================

Pre-Built Examples
------------------------------

east_asian_width
++++++++++++++++++++

Build:

.. code-block:: sh

    $ cargo build --release --example east_asian_width --no-default-features --features width

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


emoji
++++++++++++++++++++


Build:

.. code-block:: sh

    $ cargo build --release --example emoji --no-default-features --features eac

Run:

.. code-block:: sh

    $ ./target/release/examples/emoji ":smiley:"
    ":smiley:" => Some("😃")
    $ ./target/release/examples/emoji ":thumbup:"
    ":thumbup:" => Some("👍")
    $ ./target/release/examples/emoji ":+1:"
    ":+1:" => Some("👍")



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



Compilation Note
========================================

In the compilation, we will download the Unicode data from internet
(or you can provide them in the ``data/`` folder).
Then, we will start parsing Unicode data to generate Rust code (lookup table).
The generated Rust code will be included in the later compilation,
and it will generate lookup table with perfect hash function at compile time.



Related Resource
========================================

* `Unicode® Emoji <http://unicode.org/emoji/>`_ - `Data Files <http://www.unicode.org/Public/emoji/latest/>`_
* `Unicode Technical Reports <http://www.unicode.org/reports/>`_
    - `Unicode Character Database <http://www.unicode.org/reports/tr44/>`_ - `Data Files <http://www.unicode.org/Public/UCD/latest/ucd/>`_
* `Emoji Alpha Codes <https://github.com/Ranks/emoji-alpha-codes>`_ - `Data File <https://github.com/Ranks/emoji-alpha-codes/raw/master/eac.csv>`_
* `Python - unicodedata — Unicode Database <https://docs.python.org/3/library/unicodedata.html>`_


Notice
========================================



License
========================================



Special Thanks
========================================

* `Rust-PHF <https://github.com/sfackler/rust-phf>`_ - for compile time lookup tables generation
