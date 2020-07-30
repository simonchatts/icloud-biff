# How iCloud photo sharing web view seems to work

The photo shared album id is something like `V8eTzrWZCGbub3s` - in the public URL it
appears as `https://www.icloud.com/sharedalbum/#Z1sOeuVDDAuvn1a`.

---
## Step 1: list all photos

Do a POST request to `https://p37-sharedstreams.icloud.com/<album
id>/sharedstreams/webstream` with body `{"streamCtag": null}`. This returns a
big JSON document listing all photos. In particular, picking out a subset of fields:

```
{
    "photos": [
      {
        "photoGuid": "4W7QY0K2-5033-18IK-9709-O687W7D1T2Z8"
        "derivatives": {
          "384":  {
            "width": "256",
            "height": "384",
            "checksum": "11i767o85326x9l9995k9c94cm75j6v4q786947q3c"
          },
          "2304": {
            "width": "1536",
            "height": "2304",
            "checksum":  "3897p5jh3384854ppeu12dcv5yg398t977t68g90g8"
          }
        }
      }
      ...
    ]
}
```

---
## Step 2: get thumbnail URLs

Put the "photoGuid" fields of interest into a POST request to `https://p37-sharedstreams.icloud.com/O1qWlfYZDJqcz6w/sharedstreams/webasseturls` with request body":

```
{
  "photoGuids": [
    "B0E97995-Q218-123W-3915-81076LZ44U30",
    "9W5IM7J5-6869-80DV-1567-B458V4A9W0F5",
    ...
  ]
}
```

---
## Step 3: download thumbnails

The previous request returns a JSON document including:

```
{
  "items": {
    "24m512q62978q4a5137e2s04fp61o2l3d207654m3e": {
      "url_location": "cvws.icloud-content.com",
      "url_path": "/S/YxaNbdnwvnTDkF6srxnJf6F...CKYaQVw4nJ4R"
    },
    ...
  }
}
```

The key is the checksum (so it returns all listed resolutions, and you get to
choose). There is actually a `locations` bit formally specifying how to turn the
`url_path` / `url_location` into a URL:

```
  "locations": {
        "cvws.icloud-content.com": {
            "scheme": "https",
            "hosts": [
                "cvws.icloud-content.com"
            ]
        }
    },
```

but seemingly you can just stick them together with an https and cross your fingers. A
GET request to this retrieves the requested image at the specified resolution.

---
## Step 4: link to the full-resolution image

The full-size image is available at
`https://www.icloud.com/sharedalbum/#<album-id>;<photo-guid>`

eg `https://www.icloud.com/sharedalbum/#W8wTwfDXIRobq9z;3Z6SF2W8-9897-31UI-9672-S908Z0P4Y3D0`

---
## Example from command-line

```
curl -d '{"streamCtag": null}' https://p37-sharedstreams.icloud.com/F7kWxcZNHZhwo2l/sharedstreams/webstream

curl -d '{"photoGuids": ["1D9UP0K3-0312-63XZ-5756-W800I8W9C6F3"]}' https://p37-sharedstreams.icloud.com/N5qHvgMLMOmho0r/sharedstreams/webasseturls

curl 'https://cvws.icloud-content.com/S/SZ4RHjB9mAcfIRrJYsiWZ3-cYkgV/RJH96846.jpg?
      o=OzyZEImgOdxT4tL3l0WfAsQeM4Qtl7vBi-gMvzHIiw4I&v=1&z=https%3A%2F%2Fp37-conte
      nt.icloud.com%3A443&x=1&a=CAogb3s_-oWyhDZ1oWfIYvYKY1zEBGDTaFOFjIKQI7wW12pWGi
      Qzl-bgxh4FuR081zZcGjNWTcNvEysSjrufm3uBPZPehDKcMMSWq1lMoEiYiMzRjsBJQL9EWVGCQj
      aL89sEDRVkQ_mmCTYeCDpkJnmCwxPehx6sIPECfBf0jFEr06jtRSBB-MqbjN2&e=6392807985&
      r=8il4414e-yy11-6ses-5ny9-2664q9q2r697-8&s=VeaZ7j5ZcTt8UDfm1N9WIqRrXcU' \
      --output test.jpg
```

Link to full size: https://www.icloud.com/sharedalbum/#I8sXgdWIVVgwy2u;0S6RV8T8-1817-69VU-8003-R727R5L3N0Y2
