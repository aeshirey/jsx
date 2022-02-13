# jsx
`jsx` (think JSon-eXtract) is a console app for extracting properties from [JSONL](https://jsonlines.org/) data.

# Usage
Pass data to `jsx` via stdin:

```bash
$ cat my-file.json | jsx
```

`jsx` will parse each line as a self-contained JSON value. If the line is invalid, it is ignored; you can print an empty line with the `-k` flag. Therefore, to keep only valid JSON lines from your input data:

```bash
$ cat my-file.json | jsx > valid-json.json
```

The main use case is to extract values from JSON objects and/or arrays. Arguments passed to `jsx` are treated as indices to maps/arrays in order. Maps take string arguments; arrays take numeric strings. 

```bash
# Keep the value associated with the "name" key from each input object:
# Extract "adam" from {"name": "adam", address: null}
$ cat my-file.json | jsx name

# Keep the "zip" field in each input object's "address":
$ cat my-file.json | jsx address zip

# Assuming the input is an array, get the first value from each array:
$ cat my-file.json | jsx 0
```

If an object doesn't contain the specified key or if the index exceeds the array's size, the row is considered invalid and will not be output (unless `-k` is specified, then an empty line is printed). An integer argument is interpreted as both an integer index to an array (eg, `5`) and a string key to a map (eg, `"5"`).

# Example Usage
Using a few rows of [sample JSON data](https://www.appsloveworld.com/download-sample-json-file-with-multiple-records/), formatted as JSONL:

```bash
$ head -2 jsonfilewithhierarchy.json
{ "id": 2140, "title": "gj", "description": "ghj", "location": "Hermannplatz 5-6, 10967 Berlin, Germany", "lng": 0, "lat": 0, "userId": 4051, "name": "manoj", "isdeleted": false, "profilePicture": "Images/9b291404-bc2e-4806-88c5-08d29e65a5ad.png", "videoUrl": null, "images": null, "mediatype": 0, "imagePaths": null, "feedsComment": null, "commentCount": 0, "multiMedia": [ { "id": 3240, "name": "", "description": null, "url": "http://www.youtube.com/embed/mPhboJR0Llc", "mediatype": 2, "likeCount": 0, "place": null, "createAt": "0001-01-01T00:00:00" } ], "likeDislike": { "likes": 0, "dislikes": 0, "userAction": 2 }, "createdAt": "2020-01-02T13:32:16.7480006", "code": 0, "msg": null }
{ "id": 2139, "title": "dfg", "description": "df", "location": "443 N Rodeo Dr, Beverly Hills, CA 90210, USA", "lng": 0, "lat": 0, "userId": 4051, "name": "manoj", "isdeleted": false, "profilePicture": "Images/9b291404-bc2e-4806-88c5-08d29e65a5ad.png", "videoUrl": null, "images": null, "mediatype": 0, "imagePaths": null, "feedsComment": null, "commentCount": 2, "multiMedia": [ { "id": 3239, "name": "", "description": null, "url": "http://www.youtube.com/embed/RtFcZ6Bwolw", "mediatype": 2, "likeCount": 0, "place": null, "createAt": "0001-01-01T00:00:00" } ], "likeDislike": { "likes": 0, "dislikes": 0, "userAction": 2 }, "createdAt": "2020-01-02T10:54:07.6092829", "code": 0, "msg": null }

# extract the id from each row
$ head -2 jsonfilewithhierarchy.json | jsx id
2140
2139

# extract the 'likeDislike' object
$ head -2 jsonfilewithhierarchy.json | jsx likeDislike
{"dislikes":0,"likes":0,"userAction":2}
{"dislikes":0,"likes":0,"userAction":2}

# get the 'userAction' from within 'likeDislike'
$ head -2 jsonfilewithhierarchy.json | jsx likeDislike userAction
2
2

# extract the 'multimedia' array
$ head -2 jsonfilewithhierarchy.json | jsx multiMedia
[{"createAt":"0001-01-01T00:00:00","description":null,"id":3240,"likeCount":0,"mediatype":2,"name":"","place":null,"url":"http://www.youtube.com/embed/mPhboJR0Llc"}]
[{"createAt":"0001-01-01T00:00:00","description":null,"id":3239,"likeCount":0,"mediatype":2,"name":"","place":null,"url":"http://www.youtube.com/embed/RtFcZ6Bwolw"}]

# Get the first value from each `multiMedia`
$ head -2 jsonfilewithhierarchy.json | jsx multiMedia 0
{"createAt":"0001-01-01T00:00:00","description":null,"id":3240,"likeCount":0,"mediatype":2,"name":"","place":null,"url":"http://www.youtube.com/embed/mPhboJR0Llc"}
{"createAt":"0001-01-01T00:00:00","description":null,"id":3239,"likeCount":0,"mediatype":2,"name":"","place":null,"url":"http://www.youtube.com/embed/RtFcZ6Bwolw"}

# Then the 'url' from within that
$ head -2 jsonfilewithhierarchy.json | jsx multiMedia 0 url
"http://www.youtube.com/embed/mPhboJR0Llc"
"http://www.youtube.com/embed/RtFcZ6Bwolw"

# Strip the quotes (-q) from strings
$ head -2 jsonfilewithhierarchy.json | jsx multiMedia 0 url -q
http://www.youtube.com/embed/mPhboJR0Llc
http://www.youtube.com/embed/RtFcZ6Bwolw
```
