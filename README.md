# Kindlesync 

+ run kindlesync to extract clippings from kindle's clippings.txt 
into separate obsidian notes files

+ updates existing files under the "Kindlesync" heading
    + default behavior is to fullsync, ie copy over and delete
    + -a flag only adds new clippings doesn't delete any existing ones
+ if no file exists, creates a new file with "Kindlesync" heading and 
exported clippings

## features
+ per file syncing
    + specify a particular book you want to sync rather than defaulting to the 
    entirety of the books found in clippings
