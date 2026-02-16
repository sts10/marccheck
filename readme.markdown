# MARC Check
[Quickly checks](https://www.smithsonianmag.com/arts-culture/why-did-van-halen-demand-concert-venues-remove-brown-mms-from-the-menu-180982570/) given MARC record file for records in which the publication year in the 008 field does NOT match the year found in field 260 or, if not found there, field 264. 

Only checks records with a single publication year listed in field 008 ([code s](https://www.oclc.org/bibformats/en/fixedfield/dtst.html)).

(I'm a first-semester MLIS student trying to learn the structure of [a MARC record](https://loc.gov/marc/) with Rust.)

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
- [ ] Parse leader as its own Struct (consult [marc-record](https://github.com/demarque/marc-record/blob/main/src/parser.rs) and [marcli](https://github.com/hectorcorrea/marcli/blob/main/pkg/marc/leader.go))
- [ ] Write tests
- [ ] Write benchmarks (using Criterion, probably)
- [ ] Add function to output search results, maybe to a CSV?

### Optimizations to try 

- [Aho-Corasick](https://docs.rs/aho-corasick/latest/aho_corasick/)
- [memchr](https://docs.rs/memchr/latest/memchr/)
- Parallelism (Rayon)
