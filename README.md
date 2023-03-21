# rfr

A CLI for zbMATH (https://zbmath.org) for fetching the bibtex data of a publication. 

A common activity when writing articles that reference mathematics papers is to obtain the reference data in bibtex format. A convenient tool for that is zbMATH, providing a GUI for searching for articles. The CLI provides command line access to zbMATH to obtain the bibtex data quickly. 

Written in Rust.

## Example Use Cases (initial segment and segment options not implemented yet)
Suppose you want to reference the article "Being correct is not enough: efficient verification using robust linear temporal logic".

The default is to search for the search phrase as a substring of the titles. So, running
```
rfr being correct is not enough
```
will find the article. However, 
```
rfr being correct not enough
```
will find no matches. If you only remember some of the words in the title, then a loose search would be helpful, i.e.,
```
rfr being correct not enough --loosely
```

To match on the title exactly, run:
```
rfr being correct is not enough: efficient verification using robust linear temporal logic --exact 
```
To match on an initial segment run:
```
rfr being correct is not enough --init-seg
```

To match on any segment, not necessarily an initial one, run:
```
rfr correct is not enough --seg
```

In case of an inconclusive match, as in
```
rfr being correct 
```
the articles are displayed, with a certain maximum, and the user can make a choice.

## Local Features
Use 
```
rfr being correct -save=file_name
```
to save the GET response locally to a file. Exits automatically upon successful writing.

To use a previously saved GET response, use
```
rfr being correct -locally=file_name
```
and here all flags and behaviour work as usual, simply working with the saved GET response rather than generating an actual GET response. 

## Upcoming work
- Refactor into separate library files.
- Implement segmenet and init-segment flags
- Set mutual exclusivity for flags.
- Add a full suite of tests.
- Add a flag to send bibtex data straight to the clipboard.
