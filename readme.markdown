# MARCCheck
[Quickly checks](https://www.smithsonianmag.com/arts-culture/why-did-van-halen-demand-concert-venues-remove-brown-mms-from-the-menu-180982570/) a given MARC record file for records in which the publication year in the 008 field does NOT match the year found in field 260 or, if not found there, field 264. 

Only checks records with a single publication year listed in field 008 ([code s](https://www.oclc.org/bibformats/en/fixedfield/dtst.html)).

(I'm a first-semester MLIS student trying to learn the structure of [a MARC record](https://loc.gov/marc/) with Rust.)

## Installing
### Using Rust and Cargo
1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Run: `cargo install marccheck` (Run this same command to upgrade MARCCheck to latest available version.)

Uninstall MARCCheck by running `cargo uninstall marccheck`.

### Latest release
Alternatively, you can get binaries from [the GitHub releases page](https://github.com/sts10/marccheck/releases).

## Resources
* [https://loc.gov/marc/umb/](https://loc.gov/marc/umb/)
* [https://github.com/hectorcorrea/marcli/](https://github.com/hectorcorrea/marcli/)
* [https://gitlab.com/pymarc/pymarc](https://github.com/hectorcorrea/marcli/)
### On date parsing
- [https://www.oclc.org/bibformats/en/fixedfield/dtst.html](https://www.oclc.org/bibformats/en/fixedfield/dtst.html)
- [https://www.oclc.org/bibformats/en/fixedfield/dates.html](https://www.oclc.org/bibformats/en/fixedfield/dates.html)
- [https://www.oclc.org/bibformats/en/2xx/264.html](https://www.oclc.org/bibformats/en/2xx/264.html)

## Test data from:
* [https://github.com/hectorcorrea/marcli/tree/main/data](https://github.com/hectorcorrea/marcli/tree/main/data)
* [this LOC page](https://loc.gov/cds/products/marcDist.php)

## To do
- [X] Use Clap to make it a proper command line tool
- [ ] Add function to output results, maybe to a CSV?
- [ ] Write tests
- [ ] Parse leader as its own Struct, for practice (consult [marc-record](https://github.com/demarque/marc-record/blob/main/src/parser.rs) and [marcli](https://github.com/hectorcorrea/marcli/blob/main/pkg/marc/leader.go))
- [ ] Write benchmarks (using Criterion, probably)

### Optimizations to try 

- [Aho-Corasick](https://docs.rs/aho-corasick/latest/aho_corasick/)
- [memchr](https://docs.rs/memchr/latest/memchr/)
- Parallelism (Rayon)
