# ci-hoover

> I like big images and I cannot lie, you other devs can't deny
>
> Github Actions (probably)

This project is to quickly remove unneeded things from github actions base
images so my CI doesn't die running out of storage space! It's not designed
to be a serious project and does some potentially questionable things to clean
up space. I wouldn't trust it and neither should you.

If that doesn't deter you then go ahead...

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

As it stands you can look in my CI how I use it as I dogfood here. Deleting
all the paths via rayon takes ~16s. Doing a `sudo rm -rf` with a smaller list 
of paths took 1m15s when I tried it on our CI which also frees up about 15GB. 

## What Questionable Things?

Oh right, that bit earlier in the readme. Well, what happens is in ci-hoover's
CI I do all the apt-get remove commands to uninstall stuff and then use strace
to track what files and folders are deleted. 

I then take this list and reduce it down to a minimal list of files and folders
that accomplishes the same goal. This is copied into the res folder and if
there's a difference then a PR is opened with the new file change.

After all, using apt-get as a fancy rm is just needlessly slow and it's a CI
machine we don't care about messing up the image too much as long as our
project builds.

## Github Marketplace

If that doesn't deter you it should just be the same to use as
[free-disk-space](https://github.com/jlumbroso/free-disk-space). I did copy
their config and what to delete as a starting point. Maybe I'll find more waste
in future...
