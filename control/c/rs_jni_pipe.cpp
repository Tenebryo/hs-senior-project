#include <jni.h>       /* where everything is defined */
#include <stdlib.h>
#include <stdbool.h>

JavaVM *jvm = NULL;     /* denotes a Java VM */
JNIEnv *env = NULL;     /* pointer to native method interface */
jclass cls;

jmethodID mids[16];

void rs_java_vm_create() {      
    JavaVMInitArgs vm_args; /* JDK/JRE 6 VM initialization arguments */
    JavaVMOption* options = (JavaVMOption*)malloc(1*sizeof(JavaVMOption));
    options[0].optionString = (char*)"-Djava.class.path=../min2phase";
    vm_args.version = JNI_VERSION_1_6;
    vm_args.nOptions = 1;
    vm_args.options = options;
    vm_args.ignoreUnrecognized = false;
    /* load and initialize a Java VM, return a JNI interface
     * pointer in env */
    JNI_CreateJavaVM(&jvm, &env, (void**)&vm_args);
    free(options);
    cls = env->FindClass("Main");
    jmethodID mid = env->GetStaticMethodID(cls, "test", "(I)V");
    /* invoke the Main.test method using the JNI */
    env->CallStaticVoidMethod(cls, mid, 100);
}

void rs_java_vm_destroy() {
    /* We are done. */
    jvm->DestroyJavaVM();
}

//Other Functions