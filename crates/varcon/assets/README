Variant Conversion Info (VarCon)
********************************

Version 2020.12.07

Copyright 2000-2020 by Kevin Atkinson (kevina@gnu.org) and Benjamin
Titze (btitze@protonmail.ch).

This package contains information to convert between American,
British, Canadian, and Australian spellings and vocabulary as well as
other variant information.

The latest version can be found at http://wordlist.aspell.net/.


File Format
===========

The main data file is varcon.txt.  It contains information on the
preferred American, British, Canadian and Australian spelling of a
word as well as other variant information.


Varcon Lines
------------

Each line contains a mapping between the various spellings of a word.
Words are tagged to indicate where the spelling is used, and each
word/tag pair is separated with a " / ".  For example in the line:
  A Cv: acknowledgment / Av B C: acknowledgement
"acknowledgment" and "acknowledgement" are two spellings of the same
word and "A", "Cv", "B", etc are the tags.  Tags are separated by
spaces and the group of tags is separated from the word with a ": ".
Here, "acknowledgment" is the preferred American spelling (as
indicated by the "A") of the word, and "acknowledgement" is the
preferred Canadian and British spelling ("B" and "C").  However the
American spelling is sometimes used in Canada (as indicated by "Cv",
where the lowercase "v" indicated a variant form) and the British
spelling is sometimes used in America (as indicated the "Av").

More generally each tag consists of a spelling category (for example
"A") followed possible by a variant indicator.  The spelling
categories are as follows:

  A: American
  B: British "ise" spelling
  Z: British "ize" spelling or OED preferred Spelling
  C: Canadian
  D: Australian
  _: Other (Variant info based on American dictionaries, never used
            with any of the above).

and the variants tags are as follows:

  .: equal
  v: variant
  V: seldom used variant
  -: possible variant, should generally not used
  x: improper variant (should not use)

The "." or equal variant tags are reserved for special cases when
there is little agreement between dictionaries or when I think the
dictionary is wrong.  The "v" indicator is used for most words marked
as variants in the dictionary.  However, some variants will be demoted
to a "V".  For example, if the variant is marked as "also" by
Merriam-Webster, or also if only some dictionaries acknowledge the
existence the variant.  "-" is used when the variant is generally not
listed is the dictionary but I could find some evidence of its use, or
when it is marked as an archaic spelling for the word.  The "x"
is used when the spelling is almost generally considered a
misspelling, and is only included for completeness.

For Australian English "v" was used for variants that are widely used,
but not preferred, and "V" for all "-or" (vs. "-our") variants and 
variants considered "chiefly US".

If there are no tags with the 'Z' spelling category on the line then
'B' implies 'Z'.  Similarly if there are no 'C' tags then 'Z' implies
'C'.  If there are no 'D' tags then 'B' implies 'D'.

Some entries may have a number after the tags, this is a column
number and will be explained later.


Varcon Clusters
---------------

For ease of reading and maintaining the data file, each line is
grouped in a cluster of closely related words.  Each cluster is
uniquely identified by a headword, which is generally the American
spelling of word on the first line of the cluster.  Each cluster is
started with a '#' and is followed by the headword with some
additional information after it.  For example the cluster for
acknowledgment is:
  # acknowledgment <verified> (level 35)
  A Cv: acknowledgment / Av B C: acknowledgement
  A Cv: acknowledgments / Av B C: acknowledgements
  A Cv: acknowledgment's / Av B C: acknowledgement's
The "<verified>" tag will be explained latter, and "(level 35)"
indicate what level in SCOWL (see http://wordlist.sourceforge.net) 
the headword is found in.  The levels generally mean the following:
  <= 35: Very common word
  <= 70: Can be found in the dictionary
     80: Likely a valid word, can likely be found in an
         unabridged dictionary
   > 80: May not even be a legal word

Earlier versions of varcon contained numerous errors.  With version
5.0 massive effort has been made to correct many of these errors.
Clusters that have undergone some form of verification (and likely
correction) are marked with "<verified>".  As of version 5.0, most
clusters with headwords word in common usage (SCOWL level 35 and
below) should now be checked, as well as many others.  No effort was
made to check clusters with headwords in SCOWL level 80 and above;
many of those entries are unlikely to be in the dictionary anyway.


Varcon Groups
-------------

Sometimes the spelling of a word depends on the usage in which case a
cluster is split into multiple groups with each group represting one
usage of a word.  Usage annotations and/or pos tags are used to
distinguish one group from another.

Usage information is given after a " | ".  For example here is part of
the cluster for prize:
  A B: prize | reward
  A B: prizes | reward
  A C: prize / B: prise | otherwise
  A C: prizes / B: prises | otherwise
which indicated than the preferred spelling of prize is always with a
"z" when meaning a reward, but otherwise is spelled with a "s" is
British English.  In the example above a brief definition of the word
is given, but often no such attempt is made, and the definition simply
consists of a number, for example:
  A B: sake | :1
  A C: sake / Av B Cv: saki | :2

A part-of-speech (POS) tag may also given after a " | ", for example:
  A B C: practice / AV Cv: practise | <N>
  A Cv: practice / AV B C: practise | <V>
POS tags are always given in the form "<POS>" and if a definition
is also given the POS info is always first.  The POS tags used are as
follows:
  <N>: Noun
  <V>: Verb
  <Adj>: Adjective
  <Adv>: Adverb
  <A>: Adjective or Adverb
  <Inj>
  <Prep>
  <abbr>


Additional Annotations
----------------------

A "(-)" before the definition indicated a rarely used or archaic form
of a word, for example:
  A B: bark | :1
  A: bark / Av B: barque | (-) ship

A "| -- pl: someword" indicates that the word is a plural and the root
is someword.

A plain "| -- pl" indicates that the word is a plural and the root is
elsewhere within the group.  It is used when one form of the plural is
the same as the root word, for example:
  _1: yak | :1
  _ 1: yaks / _V 1: yak | :1 | -- pl
  _ 1: yak's | :1

A "| --" otherwise indicates a note which gives additional context but
does not create it's own group like a definition does.

A "#" after a line indicates a comment that is often used to indicate
why.  A "##" after a cluster indicates the the comment applies to the
entire cluster, for example:
  # coloration <verified> (level 50)
  A B C: coloration / B. Cv: colouration
  A B C: colorations / B. Cv: colourations
  A B C: coloration's / B. Cv: colouration's
  ## OED has coloration as the preferred spelling and discolouration as a
  ## variant for British Engl or some reason
In the comments ODE (not to be confused with OED) stands for Oxford
Dictionary of English, "Ox" is used for any Oxford dictionary, and
"M-W" for Merriam-Webster.


Varcon Columns
--------------

Varcon does not directly expresses the relation of words within a
group as it is normally easy to derive.  For example given a simple
group of:
  A: acknowledgment / B: acknowledgement
  A: acknowledgments / B: acknowledgements
  A: acknowledgment's / B: acknowledgement's
it is clear that acknowledgments is the plural form of acknowledgment
since they are both the American spelling of a word.  While
acknowledgEments is the plural form of acknowledgEment since they are
both the British forms of a word.  Within a group each varcon line
is considered a row in a table and each entry within a line is considered
a column.  Within this group the first column is the American spelling
and the second is the British.

Sometime the column assignment unclear, when they are explicit column
numbers may be given.  For example:
  A B: caulk / Av: calk / AV Bv 1: caulking / AV 2: calking | <N> :3
  A B: caulks / Av: calks / AV Bv 1: caulkings / AV 2: calkings | <N> :3
  A B: caulk's / Av: calk's / AV Bv 1: caulking's / AV 2: calking's | <N> :3

Each column must contain exactly one spelling of the base form of a
word, however a column may contain multiple derived forms for a single
spelling of the base form, for example:
  A B D 1: amoeba / Av Dv 2: ameba
  A B D 1: amoebas / Av Bv Dv 1: amoebae / Av Dv 2: amebas / Av Dv 2: amebae
  A B D 1: amoeba's / Av Dv 2: ameba's


Additional Files
================

The file variant-also.tab contains additional mappings between various
spellings of a word which are not yet in varcon.txt.  No attempt is
made to distinguish the primary form of a word.  The file
variant-infl.tab is like variant-also.tab except that it is created
automatically from the AGID inflection database.  The file
variant-wroot.tab is like variant-infl.tab except that it also
included the root form of the word.


The file voc.tab is similar to varcon.txt but converts between
vocabulary instead of spelling.  Unlike varcon.tab it is a simple tab
separated file with the fields corresponding to the American, British,
and Canadian words.  If more than one word if often used to describe
the same thing the words are separated with commas.  The last column
contains additional notes on when the word is used.  Unlike varcon.txt
it is generally not suitable for automatic conversion.


The "make-variant" Perl script will combine varcon.txt,
variant-also.tab, and variant-infl.tab into one huge mapping and will
output the result to "variant.tab".  If the "no-infl" option is given
than variant-infl.tab will not be included.


The "split" script will split out the information in varcon.txt into
several word lists named as follows:
  <spelling>[-v<variant level>][-uncommon].lst
where <spelling> is one of: american, british, british_z, canadian,
common, or other.  "common" is used for words which appear in
varcon.txt, yet are used in all versions of english, such as "prize",
and "other" is used for the "_" spelling category.  The mapping from
the variant indicators in varcon.txt to the numeric variant level is
as follows:
  v => 0
  V => 1
  - => 2
"-uncommon" is used for forms marked with "(-)" as already described.


The "translate" Perl script will translate a text file from one
spelling to another. Its usage is:

translate <options> [<translation array>] <from> <to>
<options> is any of
  -?,-h,--help this screen
  -m,--mark     mark words where the translation is questionable
  -i,--include  include words where the translation is questionable
<translation array> is the file name of the translation array,
                    defaults to "abbc.tab".
<from> and <to> are one of: american, british, british_z, or canadian.
british-ise and british-ize can also be used.

Text is read in from standard input and is outputted to standard out.
Words are marked with a '?' before and after the questionable word
when the option is enabled.


The file varcon.pm contains some library routines for parsing
varcon.txt and is used by many of the scripts above.


Feedback
========

If you discover any errors in these mappings or have suggestions for
additions please file a bug report at
https://github.com/kevina/wordlist/issues, or alternatively email me
directly at kevina@gnu.org, but I will likely tell you to file a bug
report so that I don't forget about it.


Sources
=======

These mappings were compiled from numerous sources.

The abc.tab was originally created from the American and British word
lists found in the Ispell distribution and the Canadian word list
created by Garst R. Reese <reese@isn.net>:

  What I have discovered is that Canadian is a modification of British.
  Canadians use ize ization, izing izable like Americans, and gram instead
  of gramme. The one exception I found was practise. It does not go to
  practize.  Otherwise they use British spelling. So, what I am currently
  checking books with is a an edited version of British, where I have
  changed all occurrences of ise to ize, isab to izab, isation to ization,
  ising to izing, and gramme to gram except I allow programme, which is
  sometimes proper unless you are talking about a computer program. I did
  bunches of greps to be sure these substitutions would work as expected.

Many other words have been added to abc.tab which were not in the
original Ispell word lists.

Many different web sources were consulted when crating the tables.  They
include:

  The American-British British-American Dictionary
    http://www.peak.org/~jeremy/dictionary/dictionary.html
    American and British Spelling Differences
      http://www.peak.org/~jeremy/dictionary/spellcat.html
  Dave (VE7CNV)'s Truly Canadian Dictionary of Canadian Spelling
    http://www.luther.bc.ca/~dave7cnv/cdnspelling/cdnspelling.html
  Canadian Spelling Convention
    http://imej.wfu.edu/articles/1999/1/02/demo/tutorial/canas.html
  Cornerstone's Canadian English Page
    http://www.web.net/cornerstone/cdneng.htm
  Inter-Play Translation: British/Canadian/American Spelling
    http://www.inter-play.com/translation/spel-ukus.htm
  Inter-Play Translation: British/Canadian/American Vocabulary
    http://www.inter-play.com/translation/voc-ukus.htm

As well as several online dictionaries:

  Marriam-Webster: http://www.m-w.com/
  American Heritage: http://www.bartleby.com/61/
  Cambridge (ESL): http://dictionary.cambridge.org/

In version 5.0 a massive effort to correct the numerous errors in
VarCon was done.  The primary sources used for verification were:

  Marriam-Webster: http://www.m-w.com/
  Free version of Oxford Dictionaries Online: 
    http://www.oxforddictionaries.com/
  Oxford dictionaries available via Oxford Reference Online
    (subscription service, http://www.oxfordreference.com/):
    The New Oxford American Dictionary (2nd edition, 2006)
      and sometimes: The Oxford American Dictionary of Current English (2002)
    The Concise Oxford English Dictionary (11th edition revised, 2008)
      and sometimes: The Oxford Dictionary of English (2nd edition revised, 2005)
    The Canadian Oxford Dictionary (2004)

I also used Tysto UK vs US spelling list available at:
  http://www.tysto.com/articles05/q1/20050324uk-us.shtml
to make sure I didn't leave out any information in VarCon, however any
additions from his lists where verified using the dictionaries
mentioned above as his lists contained numerous errors (such as
including archaic spellings of words)

I also made indirect use of Luke's Canadian, British and American
Spelling page available at:
  http://www.lukemastin.com/testing/spelling/cgi-bin/database.cgi?database=spelling
but only to perform some initial verification, in the end I rechecked
his data using the dictionaries above.  (However, his data is, by far,
more accurate than Tysto's)
    
In Version 2016.11.20 Benjamin Titze added support for Australian English.
The primary sources for this addition were:

  The Macquarie Dictionary: https://www.macquariedictionary.com.au/
  Style Manual: For Authors, Editors and Printers, 6th Edition. DCITA.
  University of Technology Sydney Publications Style Guide:
    http://www.gsu.uts.edu.au/publications/styleguide/spelling.html
  Style Manual, Department of Treasury and Finance, Tasmania:
    http://conference.tasa.org.au/wp-content/uploads/2015/03/Style-Manual.pdf
  Editor Australia - Style Guide: 
    http://www.editoraustralia.com/styleguide_spelling.html
  Webster in Australia (history of "our"/"or" spelling variants): 
    http://blogs.usyd.edu.au/elac/2008/01/webster_in_australia.html


Changelog
=========

From 2018.10.06 to 2020.12.07

   - Additional documentation on file format

   - Minor change in file format

   - Fix scripts to work with modern versions of Perl.

   - Various new entries

   - Additional cleanups

From 2017.08.24 to 2019.10.06

   - Added entries for: eukaryote, prokaryote, virtualization, volcanism

From 2016.11.20 to 2017.08.24

   - Typo fixes thanks to Jakub Wilk

From 2016.06.26 to 2016.11.20

   - New Australian spelling category thanks to the work of Benjamin
     Titze.

   - Various other fixes.

From 2016.01.19 to 2016.06.26

   - Fix plural of "bus".

From 2015.08.24 to 2016.01.19

   - Undo the effects of PERL_UNICODE in the translate script.

   - Other minor fixes and new entries.

From 2014.02.15 to 2015.08.24 (Aug 24, 2015)

   - Added entry for Koran/Koranic.

   - Tweaked "adviser" cluster.

   - Fix formatting problems.

From 2015.01.28 to 2014.02.15 (February 15, 2015)

   - Various new entries

From 2014.11.17 to 2015.01.28 (January 28, 2015)

   - Minor adjustments to a few entries (analytic, amid)

   - Added entry for shareable

   - Remove a junk entry (ted/taed).

From 2014.08.11 to 2014.11.17 (November 17, 2014)

   - Fix typos in README

   - Enhancement to VarCon translate script.  It will now, by default,
     filter clusters with a SCOWL level > 80.  This behavior can be
     controlled with the new "--thresh" option.

   - Remove a few junk entries.

From Revision 5.1 to Version 2014.08.11 (August 8, 2014)

   - Various corrections.  Most of them minor.  Two notable exceptions:

       - Added an entry for furor as the correct British spelling is furore

       - Fixed racket entries as Canadians still use racquet even
         though it is a British English (at least according to the
         Oxford dictionaries)

   - Other minor changes.

From Revision 5.0 to Revision 5.1 (January 6, 2010)

   - Corrected numerous errors after running various forms
     of verification on varcon.txt.

   - Reordered the clusters in varcon.txt so that they are
     mostly in alphabetic order based on the headword.
  
From Revision 4.1 to Revision 5.0 (December 27, 2010)

  - Completely new format for the main table which, in addition to
    providing the preferred spelling of a word for various forms of
    English, also records variant and other information.  To reflect
    this change, the name of the file was renamed from abbc.tab to
    varcon.txt.

  - Massive effort to verify the variant information against
    authoritative sources (mainly Oxford dictionaries).  Most entries
    for common words (SCOWL level 35 and below) have been checked
    against at least a British and Canadian dictionary.

  - Added variant information for numerous other words, even when
    there is no difference between the various forms on English.

  - Other changes corresponding to the new format.

From Revision 4 to Revision 4.1 (August 10, 2004)

  - Fixed various errors in abbc.tab

  - Removed clause 4 from the Ispell copyright with permission of Geoff
    Kuenning.

From Revision 3 to Revision 4 (August 7, 2004)

  - Added a column to "abc.tab" for the British "ize" spelling and
    renamed the file to abbc.tab.
  - Added verb forms of prize/prise to abbc.tab, removed from
    variant-also.tab

From Revision 2 to Revision 3 (January 2, 2003)

  - Added an option for not including variant-infl.tab for the
    make-variant perl script
  - Added the file variant-wroot.tab
  - Added a few entries given to me by Francis Bond and Edward Betts

From Revision 1 to Revision 2 (January 27, 2001)

  - Removed all "B" markers because I could not find any evidence for
    them
  - Corrected a few Canadian entries, especially those with the "B"
    markers
  - Added some more entries by trying fixed changes (such as ize to
    ise) to words in SCOWL and hand-checking over the ones with semi-common
    words in them. 
  - Added variant-infl.tab


Copyright
=========

Copyright 2000-2019 by Kevin Atkinson

Permission to use, copy, modify, distribute and sell this array, the
associated software, and its documentation for any purpose is hereby
granted without fee, provided that the above copyright notice appears
in all copies and that both that copyright notice and this permission
notice appear in supporting documentation. Kevin Atkinson makes no
representations about the suitability of this array for any
purpose. It is provided "as is" without express or implied warranty.

Copyright 2016 by Benjamin Titze

Permission to use, copy, modify, distribute and sell this array, the
associated software, and its documentation for any purpose is hereby
granted without fee, provided that the above copyright notice appears
in all copies and that both that copyright notice and this permission
notice appear in supporting documentation. Benjamin Titze makes no
representations about the suitability of this array for any
purpose. It is provided "as is" without express or implied warranty.

Since the original words lists come from the Ispell distribution:

Copyright 1993, Geoff Kuenning, Granada Hills, CA
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:

1. Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.
3. All modifications to the source code must be clearly marked as
   such.  Binary redistributions based on modified source code
   must be clearly marked as modified versions in the documentation
   and/or other materials provided with the distribution.
(clause 4 removed with permission from Geoff Kuenning)
5. The name of Geoff Kuenning may not be used to endorse or promote
   products derived from this software without specific prior
   written permission.

THIS SOFTWARE IS PROVIDED BY GEOFF KUENNING AND CONTRIBUTORS ``AS IS'' AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED.  IN NO EVENT SHALL GEOFF KUENNING OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
SUCH DAMAGE.
