#include <opencv2/core/core_c.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>

//cameras
CvCapture *cam1 = 0;
CvCapture *cam2 = 0;
//frames
IplImage *frm1 = 0;
IplImage *frm2 = 0;

//init
//returns 0 if there weren't any errors
int create_cameras(int c1, int c2) {
    if ( !(cam1 = cvCaptureFromCAM(c1)) ) {
        printf("Error opening camera with index %i\n", c1);
        return -1;
    }
    if ( !(cam2 = cvCaptureFromCAM(c2)) ) {
        printf("Error opening camera with index %i\n", c2);
        return -1;
    }
    return 0;
}

//make sure to release captures when done
void destroy_cameras() {
    if (cam1) cvReleaseCapture(&cam1);
    if (cam2) cvReleaseCapture(&cam2);
    
    if (frm1) cvReleaseImage(&frm1);
    if (frm2) cvReleaseImage(&frm2);
}

//capture frames to be retrieved later
//returns 0 if there were not any errors
int capture_frames() {
    //release images if they aren't null
    if (frm1) cvReleaseImage(&frm1);
    if (frm2) cvReleaseImage(&frm2);
    
    //get frames from cameras
    if ( !(frm1 = cvQueryFrame(cam1)) ) {
        printf("Error capturing frame\n");
        return -1;
    }
    if ( !(frm2 = cvQueryFrame(cam2)) ) {
        printf("Error capturing frame\n");
        return -1;
    }
    return 0;
}

int get_frame_dimensions(int *w1, int *h1, int *w2, int *h2) {
    if (!frm1) return -1;
    if (!frm2) return -1;
    
    *w1 = frm1->width;
    *h1 = frm1->height;
    *w2 = frm2->width;
    *h2 = frm2->height;
    
    return 0;
}

int get_frame_data(const *char img1, const *char img2) {
    if (!frm1) return -1;
    if (!frm2) return -1;
    
    memcopy(img1, frm1->imageData, frm1->imageSize);
    memcopy(img2, frm2->imageData, frm2->imageSize);
    
    return 0;
}



