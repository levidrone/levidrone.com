levidrone.com
=============

Source files of LeviDrone official website.

→ https://levidrone.com

Building
========

The OS is expected to be Ubuntu. Need to install Rust language, git, Python 3.

To build the site locally::

    git clone https://github.com/levidrone/levidrone.com
    cd levidrone.com
    cargo run

To open the site in a browser::

    cd site
    xdg-open http://127.0.0.1:8000 && python3 -m http.server 

License
=======

All contents are licensed under `Creative Commons Attribution 4.0 International Public License <https://creativecommons.org/licenses/by/4.0/>`__.

|cc| |by|

.. |cc| image:: /img/cc.svg
   :width: 30px
   :alt: CC

.. |by| image:: /img/by.svg
   :width: 30px
   :alt: BY

