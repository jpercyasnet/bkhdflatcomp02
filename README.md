# bkhdflatcomp02
Rust program: Read the md5sum of a file in csv of HD list and see if it is in csv of a Database. 

example:

bkhdflatcomp02 adbsorted.csv hdinitsorted.csv exclude.excfile nnnn

where nnnn is an optional input to read the hdinit.csv starting at nnnn row

adbsorted.csv is a sorted dump of a database

hdinit.csv is a hd list output from sorted output from hdbkmd5dblist

exclude.excfile is a text file which excludes files and directories.

see documentation repository for additional information
