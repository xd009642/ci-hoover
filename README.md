# ci-hoover

> I like big images and I cannot lie, you other devs can't deny
>
> Github Actions (probably)

This project is to quickly remove unneeded things from github actions base
images so my CI doesn't die running out of storage space!

As it stands you can look in my CI how I use it as I dogfood here. Deleting
all the paths via rayon takes ~12s. Doing a `sudo rm -rf` with the list of
paths took 1m15s when I tried it on our CI which also frees up about 15GB. 

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
