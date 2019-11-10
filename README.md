# FireSquirrel: Music Theory Tests for Guitar

## Examples:

Notes to frets
```
$ ./firesquirrel frets
With EADGBE tuning, what fret is C on bottom E?
> 7
Correct
```

Frets to notes
```
$ ./firesquirrel notes
With EADGBE tuning, what note is at fret 4 on D?
> F
Incorrect
> F#
Correct
```

Tuning selection
```
$ ./firesquirrel --tuning cgcfad frets
With CGCFAD tuning, what fret is G on bottom C?
...
```
