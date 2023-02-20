# Compile
javac -cp "../gson-2.10.1.jar" MyCity.java

#Run, basically can just get info about city by running ./build.sh "CityName"  
java -cp .:"../gson-2.10.1.jar" MyCity "$1"
