#include <iostream>
using namespace std;

class abstractemployee{

virtual void Ask_PROMOTION()=0;

};

class employee:abstractemployee{
private:
    string Name;
    string company;
    int age;
public:
    void setName(string name){
        Name=name;

    }
    string getname(){

        return Name;
    }
    
    void introduce_self()
    {
        cout << "Name--> " << Name << endl;
        cout << "company--> " << company << endl;
        cout << "age--> " << age << endl;
    }
    void Ask_PROMOTION(){
        if (age>30){
            cout<<"Got Promoted";
        }
        else{
            cout<<"No promotion";

        }
    }
    employee(string n, string C, int A)
    {
        Name = n;
        company = C;
        age = A;
    }
};

class Developer:public employee{

public:
    string fav_lag;
Developer(string n, string C, int A,string fav_lag):employee(n,C,A){

    fav_lag=fav_lag;
}

void fixbug(){
cout<<"Fixed bug using the "<<fav_lag<<endl;

}
};

int main()
{

    employee employee1 = employee("karthik", "ITC", 22);
    employee1.introduce_self();

/*four most important concet og OOP'S
1) Encapsulation
2) abstration
3)  inheritance
4) Polymorphism
  */  

//1) Encapsulation--->> binding data to a class
    employee1.setName("karthik");
    cout<<"Employ name is "<<employee1.getname()<<endl;

//2) abstration
    employee1.Ask_PROMOTION();

//3)  inheritance
    Developer d=Developer("karthik", "ITC", 22,"C++");;
    d.fixbug();
    d.Ask_PROMOTION();
 //4) Polymorphism



 return 0;
}