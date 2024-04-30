
# Features and Upcoming


| Feature                               | Status |
|---------------------------------------|-------|
| Server accepts posts and stores       | ✔️    |
| Ability to post from client to server | ⌛️    |
| Client retrieves posts and displays   | ❌     |
| Client IU lets you select user        | ❌     |
| A chatroom is avaliable               | ❌     |



# Architecture and Design Choices for Backend

### Data Storage

Ron files were chosen for now to speed up development.  SQL is likely best and will eventually be the storage choice.

Internal to the RON file, a HashMap was chosen because the ultimate goal of having posts on a users page means the greatest performance benefit I want is users to their posts.