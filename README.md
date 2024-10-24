# ci-hoover

> I like big images and I cannot lie, you other devs can't deny
> Github Actions (probably)

This project is to quickly remove unneeded things from github actions base
images so my CI doesn't die running out of storage space!

**THIS IS IN-PROGRESS AND NOT YET WORKING**

## Motivation

Looking at the default Ubuntu-latest image on github actions we currently have:

```
Filesystem      Size  Used Avail Use% Mounted on
/dev/root        73G   55G   18G  76% /
```

And there is an existing job for freeing disk space [here](https://github.com/jlumbroso/free-disk-space)
but it's a bash script and in my work CI takes 3-5 minutes to delete
things so we have room to run a job that generates a lot of data. I
would like to shave 3-5 minutes off a CI run so it can maybe take under
10 minutes for once.
