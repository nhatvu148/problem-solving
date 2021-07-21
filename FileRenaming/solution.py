def renameFile(newName, oldName):
    if not newName:
        return 1
    elif not oldName:
        return 0
    result = renameFile(newName, oldName[1:])
    if oldName[0] == newName[0]:
        result += renameFile(newName[1:], oldName[1:])
    return result

newname=input()
oldname=input()
print(renameFile(newname,oldname))