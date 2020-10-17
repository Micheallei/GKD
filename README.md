# GKD
* 如果在push时出现报错
  `error: src refspec main does not match any`
  `error: failed to push some refs to 'github.com:Micheallei/GKD.git'`
  可以考虑
   * 先用`git branch -a`查看分支，若本地分支名为`master`(这可能是git的某些默认行为导致的),可用 `git branch -m master main`将本地分支`master`重命名为`main`,和远程分支保持一致
   * 之后就可以正常使用`git push origin main`了