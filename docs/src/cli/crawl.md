# crawl
# The crawl command

Crawl command is used to fetch web pages.

```bash
polymath-cli crawl <URL>
```

You MUST specify a URL. This URL will be fetched and all other URLs within it will then be crawled.

#### `--depth`

When you use the `--depth` (`-d`) flag, you specify the number of pages to be fetched.

* Default: 1
* Maximum value: 100 (no maximum value in production)

#### `--robots-txt`

The `--robots-txt` flag allows you to bypass `/robots.txt`.
This can lead to failures and rate-limitations.

You **must specify a boolean** (true or false).
For more details, see the [robots.txt extension](/extension/robots.html).

#### `--path`

The `--path` (`-p`) flag lets you specify a directory path where all fetched pages will be saved as text content. Alternatively, use `--solr-address` to save pages on Apache Solr instead of hard text.

#### `--solr-address`

The `--solr-address` option allows to conect crawler on Solr collection.

Example: `--solr-address http://localhost:8983/api/collections/websites`