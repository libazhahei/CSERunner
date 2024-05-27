## Push command 
Users can send files or directories to the server via the `push` command. The user should specify the files or directories that they want to send to the remote. Before sending the file to the remote, CSERunner will check the available space on the remote. If there is no enough space on the remote, CSERunner will show an error. 
<pre>
<b>$local:echo "hello" > testing.txt</b>
<b>$local:./CSERunner</b>
<b>ls</b> #check the files in the current directory on remote 
. ..
<b>push testing.txt</b>
...check available space and send the file to remote 
<b>ls</b>
. .. testing.txt 
<b>cat testing.txt</b>
hello
</pre>
If no file or directory is given, CSERunner will send all files and directories of current directory to the remote. 
<pre>
<b>$local:ls</b>
. .. file1 file2 dir1 dir2
<b>$local:./CSERunner</b>
<b>ls</b>
. ..
<b>push</b>
...check available space and send the file to remote 
<b>ls</b>
. .. file1 file2 dir1 dir2
</pre>
If there is no enough available space on the remote, non of the file should be send to the remote and CSERunner will give an error. 
<pre>
<b>$local:ls</b>
. .. file1 file2 dir1 dir2
<b>$local:./CSERunner</b>
<b>ls</b>
. ..
<b>push</b>
...check available space and send the file to remote 
ERROR: No enough space on remote. push command aborted 
<b>ls</b>
. .. 
</pre>
### Upload duplicate files  
To minimize the data transmission, identical files should not be uploaded. This can be done by checking the last modification date and the MD5 value of a file. When a file is first time uploaded to the remote, we need to record its last modification date and MD5 value in a cache file. The cache file should be saved to somewhere users will not get access to. For example: `/CSERnner/cache`. As the file is being uploaded again, we check its cored in the cache file. If both metadata are identical since the last upload, the upload of that file is aborted. Otherwise, we upload the file and update its metadata. 

**Problems:**
1. How should we design the cache file? Should we store the data in text form, daump file form or binary form?
2. What if a file is modified on the remote machine? How do we update the metadata? 
3. If the uploaded files are relatively small, we potentially store more metadata than the actual data.
4. How do we match the metadata with the file? Using Path+MD5+date? This also include searching the metadata. We must include the path somewhere because the change of having files with same name is very common. 
5. Is the metadata only saved on local machine? 