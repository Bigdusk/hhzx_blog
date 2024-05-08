/*
SQLyog Community v13.1.6 (64 bit)
MySQL - 8.2.0 : Database - hhzx_blog
*********************************************************************
*/

/*!40101 SET NAMES utf8 */;

/*!40101 SET SQL_MODE=''*/;

/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;
CREATE DATABASE /*!32312 IF NOT EXISTS*/`hhzx_blog` /*!40100 DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci */ /*!80016 DEFAULT ENCRYPTION='N' */;

USE `hhzx_blog`;

/*Table structure for table `article` */

DROP TABLE IF EXISTS `article`;

CREATE TABLE `article` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '文章ID，自增主键',
  `user_id` bigint DEFAULT NULL COMMENT '用户ID，外键引用user表，表示该文章的作者',
  `category_id` bigint DEFAULT NULL COMMENT '分类id',
  `article_tags` varchar(255) DEFAULT NULL COMMENT '所有标签',
  `title` varchar(255) NOT NULL COMMENT '文章标题，不能为空',
  `cover` varchar(255) DEFAULT NULL COMMENT '封面',
  `router` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '导向路由',
  `markdown` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci COMMENT '文章内容，Markdown格式，不能为空',
  `html` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci COMMENT '文章内容，HTML格式，不能为空',
  `publish_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '发布时间，默认当前时间',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '更新时间，默认当前时间',
  `status` tinyint(1) NOT NULL COMMENT '文章状态（0: 草稿，1: 已发布，2：审核），不能为空',
  `views` bigint DEFAULT '0' COMMENT '浏览量，默认为0',
  `likes` bigint DEFAULT '0' COMMENT '点赞数，默认为0',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=29 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `article` */

insert  into `article`(`id`,`user_id`,`category_id`,`article_tags`,`title`,`cover`,`router`,`markdown`,`html`,`publish_time`,`update_time`,`status`,`views`,`likes`) values 
(28,0,14,'测试,生活,代码,rust,vue,axum','文本编辑器测试',NULL,NULL,'## ? md-editor-v3\n\nMarkdown Editor for Vue3, developed in jsx and typescript, support different themes、beautify content by prettier.\n\n### ? Base\n\n**bold**, <u>underline</u>, _italic_, ~~line-through~~, superscript<sup>26</sup>, subscript<sub>1</sub>, `inline code`, [link](https://github.com/imzbf)\n\n> quote: I Have a Dream\n\n1. So even though we face the difficulties of today and tomorrow, I still have a dream.\n2. It is a dream deeply rooted in the American dream.\n3. I have a dream that one day this nation will rise up.\n\n- [ ] Friday\n- [ ] Saturday\n- [x] Sunday\n\n![Picture](https://imzbf.github.io/md-editor-rt/imgs/mark_emoji.gif)\n\n## ? Code\n\n```vue\n<template>\n  <MdEditor v-model=\"text\" />\n</template>\n\n<script setup>\nimport { ref } from \'vue\';\nimport { MdEditor } from \'md-editor-v3\';\nimport \'md-editor-v3/lib/style.css\';\n\nconst text = ref(\'Hello Editor!\');\n</script>\n```\n\n## ? Text\n\nThe Old Man and the Sea served to reinvigorate Hemingway\'s literary reputation and prompted a reexamination of his entire body of work.\n\n## ? Table\n\n| nickname | from             |\n| -------- | ---------------- |\n| zhijian  | ChongQing, China |\n\n## ? Formula\n\nInline: $x+y^{2x}$\n\n$$\n\\sqrt[3]{x}\n$$\n\n## ? Diagram\n\n```mermaid\nflowchart TD\n  Start --> Stop\n```\n\n## ? Alert\n\n!!! note Supported Types\n\nnote、abstract、info、tip、success、question、warning、failure、danger、bug、example、quote、hint、caution、error、attention\n\n!!!\n\n## ☘️ em...\n',NULL,'2024-05-08 18:48:48','2024-05-08 18:48:48',0,2,0);

/*Table structure for table `article_tags` */

DROP TABLE IF EXISTS `article_tags`;

CREATE TABLE `article_tags` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `article_id` bigint NOT NULL COMMENT '文章ID',
  `tag_id` bigint NOT NULL COMMENT '标签ID',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `article_tags` */

/*Table structure for table `category` */

DROP TABLE IF EXISTS `category`;

CREATE TABLE `category` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '分类ID，自增主键',
  `category_name` varchar(50) NOT NULL COMMENT '分类名，不能为空',
  `ioc` varchar(50) DEFAULT NULL COMMENT '图标',
  `create_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间，默认当前时间',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '更新时间，默认当前时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=16 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `category` */

insert  into `category`(`id`,`category_name`,`ioc`,`create_time`,`update_time`) values 
(13,'默认',NULL,'2024-04-29 13:37:37','2024-04-29 13:37:37'),
(14,'代码',NULL,'2024-04-30 22:41:46','2024-04-30 22:41:46'),
(15,'生活',NULL,'2024-04-30 22:41:52','2024-04-30 22:41:52');

/*Table structure for table `comment` */

DROP TABLE IF EXISTS `comment`;

CREATE TABLE `comment` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '评论ID，自增主键',
  `comment_id` bigint DEFAULT NULL COMMENT '父留言id',
  `article_id` bigint NOT NULL COMMENT '文章ID，外键引用article表，表示该评论所属的文章',
  `user_id` bigint DEFAULT NULL COMMENT '用户ID，外键引用user表，表示该评论的作者',
  `qq` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT 'qq号',
  `web_url` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '网址',
  `content` text NOT NULL COMMENT '评论内容，不能为空',
  `create_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间，默认当前时间',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '更新时间，默认当前时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=14 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `comment` */

insert  into `comment`(`id`,`comment_id`,`article_id`,`user_id`,`qq`,`web_url`,`content`,`create_time`,`update_time`) values 
(5,NULL,15,NULL,'2831828656','http://blog.hhzx.top','不错','2024-05-02 19:56:47','2024-05-02 19:56:47'),
(6,5,15,NULL,'2831828656','http://blog.hhzx.top','666','2024-05-02 19:56:57','2024-05-02 19:56:57'),
(7,NULL,26,NULL,'2831828656','/','你好','2024-05-05 09:21:47','2024-05-05 09:21:47'),
(8,7,26,NULL,'3145323736','/','你好','2024-05-05 09:22:10','2024-05-05 09:22:10'),
(9,NULL,26,NULL,'2831828656','/','测试','2024-05-06 22:58:04','2024-05-06 22:58:04'),
(10,9,26,NULL,'2831828656','/','测试','2024-05-07 18:12:37','2024-05-07 18:12:37');

/*Table structure for table `k_v` */

DROP TABLE IF EXISTS `k_v`;

CREATE TABLE `k_v` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `key` varchar(255) DEFAULT NULL,
  `value` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `k_v` */

/*Table structure for table `permissions` */

DROP TABLE IF EXISTS `permissions`;

CREATE TABLE `permissions` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '权限ID',
  `permission_name` varchar(255) NOT NULL COMMENT '权限名',
  `description` varchar(255) DEFAULT NULL COMMENT '权限描述',
  PRIMARY KEY (`id`),
  UNIQUE KEY `permission_name` (`permission_name`)
) ENGINE=InnoDB AUTO_INCREMENT=16 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `permissions` */

insert  into `permissions`(`id`,`permission_name`,`description`) values 
(0,'所有权限','*'),
(7,'文章查看','/article/select'),
(8,'标签查看','/tag/select'),
(9,'评论查看','/comment/select'),
(10,'评论添加','/comment/insert'),
(11,'分类查看','/category/select'),
(12,'文件上传','/file/upload'),
(15,'文件下载/查看','/file/download');

/*Table structure for table `roles` */

DROP TABLE IF EXISTS `roles`;

CREATE TABLE `roles` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '角色ID',
  `role_name` varchar(255) NOT NULL COMMENT '角色名',
  `description` varchar(255) DEFAULT NULL COMMENT '角色描述',
  PRIMARY KEY (`id`),
  UNIQUE KEY `role_name` (`role_name`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `roles` */

insert  into `roles`(`id`,`role_name`,`description`) values 
(0,'管理员','最高权限'),
(1,'游客','游览权限'),
(2,'用户','拥有文章投稿权限');

/*Table structure for table `roles_permissions` */

DROP TABLE IF EXISTS `roles_permissions`;

CREATE TABLE `roles_permissions` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `roles_id` bigint DEFAULT NULL COMMENT '角色id',
  `permissions_id` bigint DEFAULT NULL COMMENT '权限id',
  PRIMARY KEY (`id`),
  UNIQUE KEY `roles_id` (`roles_id`,`permissions_id`)
) ENGINE=InnoDB AUTO_INCREMENT=147 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `roles_permissions` */

insert  into `roles_permissions`(`id`,`roles_id`,`permissions_id`) values 
(0,0,0),
(130,1,7),
(131,1,8),
(125,1,9),
(129,1,10),
(126,1,11),
(128,1,12),
(127,1,14),
(141,2,7),
(144,2,8),
(145,2,9),
(146,2,10),
(142,2,11),
(143,2,12),
(138,4,7),
(137,4,8),
(132,4,9),
(136,4,10),
(133,4,11),
(135,4,12);

/*Table structure for table `tags` */

DROP TABLE IF EXISTS `tags`;

CREATE TABLE `tags` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '标签ID',
  `tag_name` varchar(255) NOT NULL COMMENT '标签名',
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `tag_name` (`tag_name`)
) ENGINE=InnoDB AUTO_INCREMENT=32 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `tags` */

insert  into `tags`(`id`,`tag_name`,`created_at`,`updated_at`) values 
(26,'测试','2024-05-08 18:48:48','2024-05-08 18:48:48'),
(27,'生活','2024-05-08 18:48:48','2024-05-08 18:48:48'),
(28,'代码','2024-05-08 18:48:48','2024-05-08 18:48:48'),
(29,'rust','2024-05-08 18:48:48','2024-05-08 18:48:48'),
(30,'vue','2024-05-08 18:48:48','2024-05-08 18:48:48'),
(31,'axum','2024-05-08 18:48:48','2024-05-08 18:48:48');

/*Table structure for table `timeline` */

DROP TABLE IF EXISTS `timeline`;

CREATE TABLE `timeline` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `type` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL,
  `title` varchar(50) NOT NULL,
  `content` varchar(255) DEFAULT NULL,
  `time` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=10 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `timeline` */

insert  into `timeline`(`id`,`type`,`title`,`content`,`time`) values 
(6,'info','博客程序前端架构编写',NULL,'2024-05-01 17:55:40'),
(7,'success','博客程序基本功能实现',NULL,'2024-05-07 17:55:40'),
(8,'warning','自定义界面功能暂停开发',NULL,'2024-05-08 17:55:40'),
(9,'error','暂未论文编写','痛苦','2024-05-09 17:55:40');

/*Table structure for table `user` */

DROP TABLE IF EXISTS `user`;

CREATE TABLE `user` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '用户ID，自增主键',
  `username` varchar(50) NOT NULL COMMENT '用户名，唯一且不能为空',
  `password` varchar(255) NOT NULL COMMENT '密码，应进行加密存储',
  `email` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '邮箱，唯一且不能为空',
  `nickname` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '用户' COMMENT '昵称，不能为空',
  `avatar` varchar(255) DEFAULT NULL COMMENT '头像，可为空',
  `create_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间，默认当前时间',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '更新时间，默认当前时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `username` (`username`),
  UNIQUE KEY `email` (`email`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `user` */

insert  into `user`(`id`,`username`,`password`,`email`,`nickname`,`avatar`,`create_time`,`update_time`) values 
(0,'hhzx','123456','2831828656@qq.com','hhzx','http://q1.qlogo.cn/g?b=qq&nk=2831828656&s=100','2024-04-27 18:08:29','2024-04-27 18:08:29'),
(5,'user1','123123','123@qq.com','用户',NULL,'2024-05-04 22:03:07','2024-05-04 22:03:07');

/*Table structure for table `user_roles` */

DROP TABLE IF EXISTS `user_roles`;

CREATE TABLE `user_roles` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `user_id` bigint NOT NULL COMMENT '用户ID',
  `role_id` bigint NOT NULL COMMENT '角色ID',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `user_roles` */

insert  into `user_roles`(`id`,`user_id`,`role_id`) values 
(1,0,0),
(2,5,2);

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;
