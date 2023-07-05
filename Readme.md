# FileMover or the software to move bibliographies

This script moves only PDFs stored in a folder to the parent folder for easier access. Other files are deleted.
It is intended to be used when exporting libraries with the software [Zotero](https://www.zotero.org/).


<p align="center">
  <img src="./images/main.png"  width="275"/>
</p>


## Quick use

This script move file from child folder to parent folder.

<table>
<thead>
  <tr>
    <th>Before</th>
    <th>After</th>
  </tr>
</thead>
<tbody>
<tr>
<td>
    
```bash                         
files
    ├───1210
    │       A.pdf
    ├───1264
    │       B.html  
    ├─── ***
    └───C.pdf
```
</td>
<td>

```bash 
files
    ├───A.pdf
    ├───B.html  
    ├─── ***
    └───C.pdf


``` 
</td>
</tr>
</tbody>
</table>

Only select the parent file, and click `Process files`!

<p align="center">
  <img src="./images/Animation.gif" width="80%"/>
</p>

## Generate folder with the PDFs - Zotero

* Select all the PDFs you want to export.
* Right click on it, and select `Export items...`.
* Check the mark `Export Files`. (Default format `BibLaTeX` is good to go).
* Now you can select the folder `files` with the tool.


## Credits

<div align="center">

| Libraries  | Version |
| ---------- | ------- |
| eframe     | 0.22.0  |
| egui       | 0.22.0  |
| env_logger | 0.10.0  |
| open       | 5.0.0   |
| rfd        | 0.11.4  |

</div>

