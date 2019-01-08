# How To mdBook
If you're curious about building your own book and deploying it to GitHub Pages, here's a quick rundown on how I did it in 2018.

### Step 1
- Go to: https://rust-lang-nursery.github.io/mdBook/index.html
- Read everything.
- This is the template used by all of the Rust books.

### Step 2
- Create a GitHub repo for your book.
- Go to your dev environment (wherever you're building your book) and pull that new GitHub repo to your machine ```git clone LINK_TO_YOUR_NEW_REPO```.
- cd into that repo,
- mdbook init,
- name your book (it'll ask you for a name),
- Then push that newly created book back to GitHub: 
- - $git add . 
- - $git commit -m "mandatory push message"
- - $git push
- Note git push will ask for your username and password, but if you have 2FA enabled you get the extra treat of generating an access token to use instead of your password, so have fun with that (settings / developer settings / generate new token / click the first "repo" box).
- Also there's some enlightened commentary here that might help: https://www.reddit.com/r/programminghorror/comments/4qsywz/rant_git_sucks/
- If that didn't work, I dunno ¯\\_(ツ)_/¯ but I feel for you deeply.

### Step 3
- Go to: https://travis-ci.org/
- Login with GitHub and give it access to all the things, and make sure you click the sync icon for your project. This is what builds and pushes the book to GitHub Pages. I know nothing else about it. There's probably other ways to deploy your book, but I don't know about them.
- Create a .travis.yml file in your main directory (just like this one here: https://github.com/burrrata/The-Grin-Book/blob/master/.travis.yml)
- Now here's the tricky part, the $GITHUB_ACCESS_TOKEN. 
Go to the "more options" tab on your TravisCI page for the project, select settings, and scroll down to Environment Variables. 
This is where the magic happens... but first! Open a new tab and go back to GitHub. Do not pass GO. Do not collect 200. Ok... great. 
Now in your global account settings (click on your avatar in the top right and settings from the drop down menu) you'll see a thing called "developer settings" all the way under the other settings (as if everything on GitHub wasn't "developer" settings). Click on that. 
Good... now, where it says "personal access tokens": click on that. Good! Now... create an access token, copy that value (a long random number), and go back to the TravisCI settings page! Where it says Environment Variables give it a name like... GITHUB_TOKEN and paste the key into the value field. Then look to your right and click "add". Good!   
Now go back to your .travis.yml file and add $GITHUB_TOKEN (or whatever you named it) to the github-token section under the deploy section. Note, the $ is important! It must be there right before the letters!
Go through the ```git add .```, ```git commit -m "stuff"```, ```git push``` process.
Now go back to TravisCI, and it should be building. While it does... pray. If it doesn't work, try doing what I did and ask the friendly folks at https://users.rust-lang.org/. If you post to this thread I might even see it and respond (key word might) https://users.rust-lang.org/t/mdbook-0-1-0-a-markdown-document-generator/15248/4
- Also, if your .travis.yml file isn't working try to copypasta the file from other Rust books and see if one of them works.
- IF you successfully pushed your book and linked it to Travis, this now means that you can edit the book directly from the GitHub website (assuming you follow the structure of mdBook) and it will build the book for you. This means playing in the [Rust Playground](https://play.rust-lang.org/) and/or rolling html for the pages on codepen or jsbin, and then just pasting it in rather than dealing with the mdBook CLI interface. 

# Good luck!

Oh and here's some other potentially helpful stuff
- https://github.com/rust-lang-nursery/mdBook
- https://rust-lang-nursery.github.io/mdBook/
- https://github.com/rust-lang/rust-by-example

Publishing to GitHub Pages
- https://www.reddit.com/r/programminghorror/comments/4qsywz/rant_git_sucks/
- https://rust-lang-nursery.github.io/mdBook/continuous-integration.html
- https://docs.travis-ci.com/user/deployment/pages/

Editing and Writing
- https://rust-lang-nursery.github.io/mdBook/format/theme/syntax-highlighting.html
- https://rust-lang-nursery.github.io/mdBook/format/theme/editor.html
- https://rust-lang-nursery.github.io/mdBook/format/mathjax.html
