#!/bin/bash

# this script aims to delete all '.git' & '.gitignore' files/folders in each workspace
# so that there is only one `.git` in the root folder
# make file exectuable using `chmod +x file_name.sh`


# change to the root directory
cd ..
DIRECTORY=$PWD

GIT_FILE_1=".git"
GIT_FILE_2=".gitignore"

# loop through the directories
for FOLDER in "$DIRECTORY"/*; do
	if [ -d "$FOLDER" ]; then
		echo "➡️ $(basename "$FOLDER")"
		cd $FOLDER

		# delete .git folder
		if find $FOLDER -type d -name "$GIT_FILE_1" -print -quit | grep -q .; then
			rm -rf $GIT_FILE_1
			echo "✅ Deleted '.git' found in $FOLDER"
		else
			echo "❗No '.git' file found in $FOLDER"
		fi

		# delete .gitignore file
		if find $FOLDER -type f -name "$GIT_FILE_2" -print -quit | grep -q .; then
			rm $GIT_FILE_2
			echo "✅Deleted '.gitignore' found in $FOLDER"
		else
			echo "❗No '.gitignore' file found in $FOLDER"
		fi
		# done
	fi
	echo
done

echo "💯💯 DONE!!"