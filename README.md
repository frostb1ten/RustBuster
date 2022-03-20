# RustBuster

```
Usage: ./RustBuster [Options] http://(FUZZ).website.com/ Wordlist
Options:
-h : Display this help message.
-d : Directory fuzzing
-f : Subdomain fuzzing
-vh : Virtual Host Subdomain fuzzing

```


Directory fuzzing:
```
 ./Rustbuster -d https://www.website.com/ wordlist.txt
```

Subdomain fuzzing
```
 ./Rustbuster -f https://FUZZ.website.com/ wordlist.txt
```


Vhost Subdomain fuzzing
```
 ./Rustbuster -vh http://FUZZ.website.com/ wordlist.txt
```
