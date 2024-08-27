# tp.ajhs.li

The website for the terms of service and privacy policy for ajhs.li has been updated to provide clear and accessible information for users.

For those interested in creating a minimal, elegant, and content-oriented static blog CMS, consider [Noer](https://github.com/homelyguy/noer). It is designed for users who prefer **Markdown**, **static sites**, and **WebAssembly**. Noer features a Markdown-based interface that is easy to use, supports static style, and is compatible with GitHub Pages for effortless deployment, serving, and maintenance. No JavaScript, CSS, or HTML is modified, ensuring straightforward and efficient content management.

See the [completed website for ajhs.li](https://tp.ajhs.li/).

# Development Notes

## How To Use
there are some requirements to be satisfied before we use it.

### Pre-requisite
- First of all, [rust](https://rust-lang.org) language and its building tool are necessary. you can install them following the description [here](https://www.rust-lang.org/tools/install);
-	Noer is built upon Web-Assembly and [yew](https://yew.rs), the [tutorial](https://yew.rs/docs/tutorial) is recommanded before getting started;
- build tools are needed
 -- trunk ( `cargo install trunk` )
 -- wasm32-unknown-unknown ( `rustup target add wasm32-unknown-unknown ` )


### Github Page(Optional)
By default, the compiled output is not suitable for github pages, you got to enable it in the configuration file `src/constant.rs` 
```
// if using github pages, it is required to deploy it to subpath
// specify the sub-path(ends with `/`)
// if not specified, it wont use sub-path
pub const SUBPATH: &str = "demo/";
```

### Site Infomation
There are some basic info about the site in `stc/constant.rs`, including
- `MODE` the current mode of the project, noer will set logger if in development mode and disable it is release mode.  
- `ITEMS_PER_PAGE` number of posts card that a page to display, note that it shall be multiple of 3.
- `ADMIN` username of the ower.
- `SITE_NAME` the name you want call the site.
- `LOGO_PIC`: the logo display in the page.
- `AVATR_PIC` avatar of the user
- `SITE_DESCRIPTION` just site Description.
- `USER_INFO` some extra infomation of the user present to viewers, such as social network, email, etc. Note that it is line-separated, each line is a key-value pair.

### Compile
```
//normal 
sh ./build.sh

// use trunk to preview the site
	- without sub-path (`SUBPATH` is "/")
	  trunk serve 
  - with sub-path (`SUBPATH` is **not** "/")
		trunk serve --public-url /demo
```

### Deployment
`noer` is static style site, Deploying noer as web application is just as simple as exposing the compiled `index.html`. here representing an example for nginx user
```
// /etc/nginx/sites-enabled/noer.conf
server {
	listen 8080;
	listen [::]:8080 ;

  root <absolute-path-to-project-directory>/<compiled-output-directory>;

	index index.html;

	server_name _;

	location / {
		try_files $uri $uri/ =404;
	}
}

// Note that the example above is deployed to localhost:8080/
// if you prefer to deploying to sub path `localhost:8080/demo/`
// configue the `subpath` in `src/constant.rs` 
// rename the compiled output directory to `demo`
server {
	...
  root <absolute-path-to-project-directory>;
	location /demo {
		...
	}
	...
}
```

### Known Issues
- due to the limit of yew, `late` or `katex` is not supported, see [issue](https://github.com/yewstack/yew/discussions/2446)
- some uncommon features of markdown are not supported so far (subscript/supscript, some Markup, Definition, Abbreviations)
